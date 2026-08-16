//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2009;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta593(t94473: f64, t1389: f64, t3964: f64, t92986: f64, t7028: f64, t9736: f64, t9737: f64, t26009: f64, t9802: f64, t64: f64, t9990: f64, t2482: f64, t596: f64, t7262: f64, t4021: f64, t25981: f64, t27: f64, t550: f64, t7021: f64, t25273: f64, t540: f64, t1372: f64, t2019: f64, t9951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94474, t94477, t94479, t94484, t94491, t94497) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2009(t94473, t1389, t3964, t92986, t7028, t9736, t9737, t26009, t9802, t64, t9990, t2482, t596, t7262);
        let (t94498, t94508, t94513, t94519, t94520, t94522) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2010(t4021, t94497, t2482, t25981, t27, t550, t7021, t25273, t540, t1372, t2019, t9951);
    (t94474, t94477, t94479, t94484, t94491, t94497, t94498, t94508, t94513, t94519, t94520, t94522)
}
