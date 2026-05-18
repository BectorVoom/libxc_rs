//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1240/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1240<F: Float>(t13780: F, t13859: F, t3990: F, t9326: F, t14664: F, t9270: F, t14705: F, t51666: F, t14637: F, t3974: F, t8759: F, t11375: F, t1185: F, t13924: F, t50995: F, t51053: F, t51675: F, t53134: F, t53140: F, t53152: F, t53155: F, t53158: F, t53166: F, t53170: F, t8776: F, t9697: F) -> F {
    let t53174 = t13859 * t3990 * t13780 * t9326;
    let t53177 = F::new(7.0) / F::new(72.0) * t9270 * t14664;
    let t53178 = t51666 * t14705;
    let t53179 = F::new(7.0) / F::new(576.0) * t53178;
    let t53182 = t14637 * t3990 * t3974 * t8759;
    let t53184 = t53134 / F::new(48.0) + F::new(7.0) / F::new(288.0) * t50995 - t53140 / F::new(384.0) + t8776 * t1185 * t13924 / F::new(32.0) - t9697 * t1185 * t51053 / F::new(32.0) - t11375 * t51675 / F::new(48.0) + t53152 / F::new(384.0) - t53155 - t53158 / F::new(96.0) - t53166 / F::new(384.0) + t53170 / F::new(384.0) + t53174 / F::new(768.0) - t53177 - t53179 + F::new(5.0) / F::new(768.0) * t53182;
    t53184
}
