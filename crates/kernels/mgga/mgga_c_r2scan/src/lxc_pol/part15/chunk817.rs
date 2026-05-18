//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 817/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk817<F: Float>(t51: F, t101: F, t1216: F, t1225: F, t1228: F, t2517: F, t2520: F, t2713: F, t40: F, t6995: F, t906: F, t7276: F, t552: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t7288 = piecewise3::<f64>(t52, F::new(0.0), -F::new(10.0) / F::new(27.0) * t2517 * t1225 - F::new(40.0) / F::new(9.0) * t2520 * t6995 + F::new(10.0) / F::new(9.0) * t906 * t1228 - F::new(10.0) / F::new(3.0) * t101 * t1216 + F::new(10.0) * t2713 * t40);
    let t7290 = t7276 / F::new(2.0) + t7288 / F::new(2.0);
    let t7291 = t552 * t7290;
    (t7290, t7291)
}
