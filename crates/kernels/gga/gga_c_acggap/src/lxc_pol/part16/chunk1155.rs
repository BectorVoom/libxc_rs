//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1155/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1155<F: Float>(t30468: F, t6144: F, t7433: F, t9758: F, t31224: F, t31227: F, t31230: F, t31231: F, t35397: F, t35399: F, t35400: F, t35404: F, t35408: F, t35411: F, t37538: F, t39937: F, t39939: F, t39944: F, t39946: F, t39948: F) -> F {
    let t39950 = t30468 * t6144;
    let t39952 = t7433 * t9758;
    let t39956 = -F::new(0.34299214494455789578e-2) * t39937 - F::new(0.47172138434406228102e-2) * t39939 + t35397 - t35399 + F::new(0.51448821741683684367e-2) * t35400 + t35404 - F::new(0.31448092289604152068e-2) * t39944 - F::new(0.19293308153131381637e-1) * t39946 + F::new(0.40015750243531754507e-2) * t39948 - t35408 - t35411 + F::new(0.34299214494455789578e-2) * t39950 + F::new(0.64311027177104605458e-3) * t39952 - F::new(0.2250885951198661191e-1) * t31224 + t31227 + t37538 + t31230 + F::new(0.17149607247227894789e-2) * t31231;
    t39956
}
