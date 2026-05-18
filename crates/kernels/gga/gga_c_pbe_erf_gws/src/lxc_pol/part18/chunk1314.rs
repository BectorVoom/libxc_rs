//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1314/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1314<F: Float>(t12210: F, t14121: F, t11426: F, t50998: F, t53447: F, t11430: F, t12213: F, t14747: F, t15320: F, t2408: F, t2409: F, t3066: F, t335: F, t338: F, t3907: F, t4053: F, t51651: F, t53730: F, t56776: F, t56783: F, t56787: F, t56791: F, t56793: F, t56799: F, t56811: F, t56813: F, t6781: F) -> F {
    let t56815 = t14121 * t12210;
    let t56818 = t50998 * t53447 * t11426;
    let t56821 = t50998 * t53447 * t11430;
    let t56823 = t56776 / F::new(24.0) - t335 * t338 * t3907 * t4053 / F::new(96.0) - t53730 - t56783 / F::new(48.0) + t56787 / F::new(1536.0) - t56791 / F::new(384.0) - t56793 / F::new(96.0) + t56799 / F::new(48.0) + t2408 * t2409 * t6781 * t15320 / F::new(24.0) + t3066 * t2409 * t12213 * t14747 / F::new(24.0) - F::new(35.0) / F::new(216.0) * t51651 - t56811 / F::new(768.0) + t56813 / F::new(24.0) + t56815 / F::new(8.0) + t56818 / F::new(192.0) + t56821 / F::new(192.0);
    t56823
}
