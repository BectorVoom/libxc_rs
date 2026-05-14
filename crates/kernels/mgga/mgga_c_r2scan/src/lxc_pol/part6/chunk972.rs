//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 972/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk972<F: Float>(t51: F, t101: F, t1216: F, t1225: F, t1228: F, t2517: F, t2520: F, t2713: F, t40: F, t6995: F, t906: F, t7276: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t7288 = piecewise3(t52, 0.0, -10.0 / 27.0 * t2517 * t1225 - 40.0 / 9.0 * t2520 * t6995 + 10.0 / 9.0 * t906 * t1228 - 10.0 / 3.0 * t101 * t1216 + 10.0 * t2713 * t40);
    let t7290 = t7276 / 2.0 + t7288 / 2.0;
    (t7290,)
}
