//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1290/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1290<F: Float>(t3959: F, t8723: F, t3202: F, t3955: F, t14121: F, t2409: F, t26768: F, t14113: F, t14614: F, t2242: F, t4161: F, t14742: F, t840: F) -> (F, F, F, F, F, F) {
    let t53968 = t3959 * t8723;
    let t53970 = t3955 * t3202;
    let t53971 = F::new(7.0) / F::new(144.0) * t53970;
    let t53973 = t14121 * t2409 * t26768;
    let t53975 = t14113 * t14614;
    let t53976 = F::new(7.0) / F::new(576.0) * t53975;
    let t53977 = t2242 * t4161;
    let t53980 = F::new(7.0) / F::new(144.0) * t840 * t14742;
    (t53968, t53971, t53973, t53976, t53977, t53980)
}
