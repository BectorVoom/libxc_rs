//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta795 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2875;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2876;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta795(t15397: f64, t41583: f64, t2874: f64, t2918: f64, t4632: f64, t15534: f64, t3022: f64, t1100: f64, t3329: f64, t15537: f64, t3007: f64, t981: f64, t11396: f64, t4719: f64, t15566: f64, t5023: f64, t52170: f64, t52174: f64, t52176: f64, t52178: f64, t52180: f64, t41832: f64, t4732: f64, t11524: f64, t15525: f64, t11299: f64, t11300: f64, t1610: f64, t11112: f64, t15101: f64, t11116: f64, t15421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52182, t52185, t52187, t52188, t52194) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2875(t15397, t41583, t2874, t2918, t4632, t15534, t3022, t1100, t3329, t15537, t3007, t981);
        let (t52196, t52197) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2876(t11396, t4719, t15566, t5023, t52170, t52174, t52176, t52178, t52180, t52182, t52185, t52187, t52188, t52194);
        let (t52201, t52204, t52207, t52209, t52211) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2877(t41832, t4732, t981, t11524, t15525, t11299, t11300, t1610, t11112, t15101, t11116, t15421);
    (t52182, t52185, t52187, t52194, t52196, t52197, t52201, t52204, t52207, t52209, t52211)
}
