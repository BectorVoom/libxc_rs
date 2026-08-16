//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1880;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta561(t27349: f64, t689: f64, t25260: f64, t4368: f64, t820: f64, t844: f64, t4462: f64, t92951: f64, t92963: f64, t92966: f64, t92969: f64, t27253: f64, t9775: f64, t14833: f64, t240: f64, t2661: f64, t7043: f64, t14857: f64, t25234: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98892, t98937, t98949, t98960, t98961, t98962, t98964) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1880(t27349, t689, t25260, t4368, t820, t844, t4462, t92951, t92963, t92966, t92969, t27253, t9775);
        let (t98968, t98972, t98976, t98979) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1881(t14833, t240, t2661, t7043, t14857, t25234, t25240, t2710, t4371, t10744, t4353, t7028);
    (t98892, t98937, t98949, t98960, t98961, t98962, t98964, t98968, t98972, t98976, t98979)
}
