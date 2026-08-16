//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1155/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1155(t30468: f64, t6144: f64, t7433: f64, t9758: f64, t31224: f64, t31227: f64, t31230: f64, t31231: f64, t35397: f64, t35399: f64, t35400: f64, t35404: f64, t35408: f64, t35411: f64, t37538: f64, t39937: f64, t39939: f64, t39944: f64, t39946: f64, t39948: f64) -> f64 {
    let t39950 = t30468 * t6144;
    let t39952 = t7433 * t9758;
    let t39956 = -0.34299214494455789578e-2_f64 * t39937 - 0.47172138434406228102e-2_f64 * t39939 + t35397 - t35399 + 0.51448821741683684367e-2_f64 * t35400 + t35404 - 0.31448092289604152068e-2_f64 * t39944 - 0.19293308153131381637e-1_f64 * t39946 + 0.40015750243531754507e-2_f64 * t39948 - t35408 - t35411 + 0.34299214494455789578e-2_f64 * t39950 + 0.64311027177104605458e-3_f64 * t39952 - 0.2250885951198661191e-1_f64 * t31224 + t31227 + t37538 + t31230 + 0.17149607247227894789e-2_f64 * t31231;
    t39956
}
