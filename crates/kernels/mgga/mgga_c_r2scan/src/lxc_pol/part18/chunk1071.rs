//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1071/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1071<F: Float>(t2184: F, t30281: F, t3308: F, t10810: F, t1577: F, t9296: F, t3602: F, t40062: F, t8089: F, t3606: F, t40066: F, t11670: F, t2124: F, t29764: F, t11705: F, t7313: F) -> (F, F, F, F, F, F) {
    let t43500 = t2184 * t3308 * t30281;
    let t43503 = t1577 * t10810 * t9296;
    let t43506 = t40062 * t3602 * t8089;
    let t43509 = t40066 * t3606 * t8089;
    let t43512 = t11670 * t2124 * t29764;
    let t43514 = t7313 * t11705;
    (t43500, t43503, t43506, t43509, t43512, t43514)
}
