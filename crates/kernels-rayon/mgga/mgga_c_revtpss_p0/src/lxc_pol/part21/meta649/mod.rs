//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2434;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta649(t367: f64, t371: f64, t373: f64, t9291: f64, t1058: f64, t11907: f64, t3197: f64, t3201: f64, t11962: f64, t3231: f64, t11973: f64, t11904: f64, t11773: f64, t11865: f64, t11941: f64, t11942: f64, t127: f64, t11937: f64, t11947: f64, t3205: f64, t3206: f64, t676: f64, t11643: f64, t11994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42121, t42122, t42124, t42139, t42141, t42146, t42149) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2434(t367, t371, t373, t9291, t1058, t11907, t3197, t3201, t11962, t3231, t11973, t11904);
        let (t42155, t42170, t42172, t42176, t42190) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2435(t11773, t11865, t11941, t11942, t127, t371, t11937, t11947, t3205, t3206, t676, t11643, t11994);
    (t42121, t42122, t42124, t42139, t42141, t42146, t42149, t42155, t42170, t42172, t42176, t42190)
}
