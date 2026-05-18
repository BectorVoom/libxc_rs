//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 794/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk794<F: Float>(t12493: F, t5061: F, t12513: F, t657: F, t10517: F, t12499: F, t12503: F, t12507: F, t12511: F, t25: F, t5047: F, t5082: F, t7239: F, t7269: F) -> (F, F, F) {
    let t12855 = t5061 * t12493;
    let t12858 = t657 * t12513;
    let t12868 = -t5047 - F::new(0.29629629629629629629e-2) * t25 * t12855 - F::new(0.66666666666666666667e-2) * t25 * t12858 + F::new(0.44444444444444444445e-2) * t10517 + F::new(0.14396666666666666667e0) * t12499 - F::new(0.71983333333333333335e-1) * t12503 - F::new(0.21595e0) * t12507 + F::new(0.21595e0) * t12511 - F::new(0.22222222222222222222e-1) * t7239 - t5082 - F::new(0.47988888888888888888e-1) * t7269;
    (t12855, t12858, t12868)
}
