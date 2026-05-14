//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1194/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1194<F: Float>(t14001: F, t15334: F, t2409: F, t36000: F, t3959: F, t14673: F, t2503: F, t13796: F, t15167: F, t3989: F, t53539: F, t11633: F, t53710: F, t56296: F, t14682: F, t3140: F, t57321: F) -> (F, F, F, F, F, F) {
    let t57542 = t14001 * t15334;
    let t57545 = t3959 * t2409 * t36000;
    let t57551 = t14673 * t2503;
    let t57555 = t3989 * t13796 * t53539 * t15167;
    let t57570 = t3989 * t53710 * t56296 * t11633;
    let t57574 = t3989 * t14682 * t57321 * t3140;
    (t57542, t57545, t57551, t57555, t57570, t57574)
}
