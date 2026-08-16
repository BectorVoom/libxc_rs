//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1085/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1085(t38145: f64, t6085: f64, t6087: f64, t2161: f64, t5148: f64, t37638: f64, t2111: f64, t6461: f64, t6072: f64, t6064: f64, t6093: f64, t10698: f64, t10805: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38147 = t6085 * t38145 * t6087;
    let t38149 = t2161 * t5148;
    let t38150 = t38149 * t37638;
    let t38152 = t2111 * t6461;
    let t38153 = t38152 * t6072;
    let t38156 = t6093 * t38145 * t6064;
    let t38158 = t10698 * t10805;
    (t38147, t38149, t38150, t38152, t38153, t38156, t38158)
}
