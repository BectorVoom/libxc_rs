//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta920 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2969;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta920(t19147: f64, t4719: f64, t23694: f64, t2986: f64, t974: f64, t981: f64, t77863: f64, t964: f64, t973: f64, t19468: f64, t19134: f64, t78094: f64, t78096: f64, t78099: f64, t78154: f64, t78192: f64, t78195: f64, t78201: f64, t78203: f64, t78206: f64, t78246: f64, t78248: f64, t78251: f64, t78254: f64, t78472: f64, t78474: f64, t4711: f64, t64504: f64, t23811: f64, t300: f64, t983: f64, t52238: f64, t78423: f64, t18898: f64, t52459: f64, t15258: f64, t19133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78686, t78690, t78694, t78696, t78698, t78699) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2969(t19147, t4719, t23694, t2986, t974, t981, t77863, t964, t973, t19468, t19134, t78094, t78096, t78099, t78154, t78192, t78195, t78201, t78203, t78206, t78246, t78248, t78251, t78254, t78472, t78474);
        let (t78703, t78706, t78709, t78712, t78715) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2970(t4711, t64504, t981, t23811, t300, t983, t52238, t78423, t18898, t52459, t15258, t19133);
    (t78686, t78690, t78694, t78696, t78698, t78699, t78703, t78706, t78709, t78712, t78715)
}
