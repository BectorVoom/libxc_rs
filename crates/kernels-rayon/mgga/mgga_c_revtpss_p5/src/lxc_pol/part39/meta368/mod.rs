//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1295;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta368(t15525: f64, t4733: f64, t981: f64, t15495: f64, t300: f64, t15234: f64, t964: f64, t973: f64, t2986: f64, t4707: f64, t974: f64, t11506: f64, t1633: f64, t11509: f64, t2988: f64, t15100: f64, t15103: f64, t15377: f64, t15379: f64, t15382: f64, t15385: f64, t15388: f64, t15392: f64, t15395: f64, t15399: f64, t15519: f64, t15522: f64, t15524: f64, t3329: f64, t5023: f64, t5024: f64, t4682: f64, t983: f64, t3030: f64, t4719: f64, t3034: f64, t11591: f64, t1642: f64, t11524: f64, t4732: f64, t2989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15528, t15530, t15536, t15540, t15541) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1294(t15525, t4733, t981, t15495, t300, t15234, t964, t973, t2986, t4707, t974, t11506, t1633);
        let (t15545, t15546) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1295(t11509, t2988, t15541, t981, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t3329, t5023, t5024);
        let (t15549, t15551, t15553, t15555, t15558, t15559) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1296(t300, t4682, t983, t3030, t4719, t3034, t11591, t1642, t11524, t4732, t981, t2989);
    (t15528, t15530, t15536, t15540, t15545, t15546, t15549, t15551, t15553, t15555, t15558, t15559)
}
