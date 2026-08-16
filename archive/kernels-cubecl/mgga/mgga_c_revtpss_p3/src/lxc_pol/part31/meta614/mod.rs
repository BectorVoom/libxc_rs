//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2057;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta614<F: Float>(t25411: F, t98877: F, t27349: F, t689: F, t92843: F, t92838: F, t27341: F, t93342: F, t93364: F, t27194: F, t887: F, t1580: F, t2439: F, t25334: F, t25260: F, t4368: F, t820: F, t844: F, t4462: F, t92951: F, t27253: F, t9775: F, t14833: F, t240: F, t2661: F, t7043: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98881, t98894, t98897, t98907, t98911, t98918, t98920) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2057::<F>(t25411, t98877, t27349, t689, t92843, t92838, t27341, t93342, t93364, t27194, t887, t1580, t2439, t25334);
        let (t98937, t98950, t98964, t98968) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2058::<F>(t25260, t4368, t820, t844, t4462, t92951, t27253, t9775, t14833, t240, t2661, t7043);
    (t98881, t98894, t98897, t98907, t98911, t98918, t98920, t98937, t98950, t98964, t98968)
}
