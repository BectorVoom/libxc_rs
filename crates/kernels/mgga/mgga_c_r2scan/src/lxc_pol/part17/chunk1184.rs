//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1184/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1184<F: Float>(t11659: F, t7601: F, t2184: F, t30281: F, t3308: F, t10810: F, t1577: F, t9296: F, t3602: F, t40062: F, t8089: F, t3606: F, t40066: F) -> (F, F, F, F, F) {
    let t43497 = t7601 * t11659;
    let t43500 = t2184 * t3308 * t30281;
    let t43503 = t1577 * t10810 * t9296;
    let t43506 = t40062 * t3602 * t8089;
    let t43509 = t40066 * t3606 * t8089;
    (t43497, t43500, t43503, t43506, t43509)
}
