//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2064;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta624(t25411: f64, t98877: f64, t27349: f64, t689: f64, t92843: f64, t92838: f64, t27341: f64, t93342: f64, t93364: f64, t27194: f64, t887: f64, t1580: f64, t2439: f64, t25334: f64, t25260: f64, t4368: f64, t820: f64, t844: f64, t4462: f64, t92951: f64, t27253: f64, t9775: f64, t14833: f64, t240: f64, t2661: f64, t7043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98881, t98894, t98897, t98907, t98911, t98918, t98920) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2064(t25411, t98877, t27349, t689, t92843, t92838, t27341, t93342, t93364, t27194, t887, t1580, t2439, t25334);
        let (t98937, t98950, t98964, t98968) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2065(t25260, t4368, t820, t844, t4462, t92951, t27253, t9775, t14833, t240, t2661, t7043);
    (t98881, t98894, t98897, t98907, t98911, t98918, t98920, t98937, t98950, t98964, t98968)
}
