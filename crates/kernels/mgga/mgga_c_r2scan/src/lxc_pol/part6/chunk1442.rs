//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1442/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1442<F: Float>(t51: F, t2713: F, t409: F, t101: F, t23869: F, t23872: F, t23878: F, t23881: F, t2517: F, t2520: F, t40: F, t419: F, t4921: F, t4927: F, t7073: F, t7076: F, t7081: F, t906: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t27164 = 40.0 * t2713 * t409;
    let t27166 = piecewise3(t52, 0.0, 40.0 / 81.0 * t7073 * t4921 + 20.0 / 9.0 * t7076 * t23869 - 10.0 / 9.0 * t2517 * t23872 - 20.0 / 3.0 * t7081 * t419 + 20.0 * t2520 * t23878 - 20.0 / 3.0 * t2520 * t23881 + 10.0 / 9.0 * t906 * t4927 + 20.0 * t101 * t40 - t27164);
    (t27166,)
}
