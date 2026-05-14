//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1112/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1112<F: Float>(t1162: F, t13796: F, t2195: F, t3989: F, t3307: F, t875: F, t14127: F, t2503: F, t14803: F, t4414: F, t13888: F, t13925: F, t22343: F, t2408: F, t2410: F, t4002: F, t51526: F, t53614: F, t53617: F, t53623: F, t53626: F, t53629: F, t53631: F, t53636: F, t6793: F, t8754: F, t9265: F, t9283: F) -> (F,) {
    let t53639 = t3989 * t13796 * t1162 * t2195;
    let t53643 = t3989 * t13796 * t3307 * t875;
    let t53645 = t14127 * t2503;
    let t53646 = 7.0 / 144.0 * t53645;
    let t53656 = 7.0 / 36.0 * t4414 * t14803;
    let t53657 = t6793 * t53617 / 24.0 + t22343 * t13925 / 48.0 - t53623 / 1536.0 + t53626 + 7.0 / 2304.0 * t51526 + t53629 - t53631 / 384.0 - t9265 * t4002 / 96.0 + t53636 + t53639 / 3072.0 + t53643 / 1536.0 - t53646 - t2408 * t9283 * t13888 * t8754 / 12.0 - t2408 * t9283 * t53614 * t2410 / 12.0 + t53656;
    (t53657,)
}
