//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1583;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1584;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta538(t1432: f64, t22964: f64, t686: f64, t72: f64, t14239: f64, t22332: f64, t10023: f64, t22863: f64, t14141: f64, t23037: f64, t22336: f64, t13790: f64, t6843: f64, t10022: f64, t2782: f64, t1882: f64, t6888: f64, t22857: f64, t555: f64, t22953: f64, t22954: f64, t4101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86374, t86377, t86381, t86401, t86411, t86413) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1583(t1432, t22964, t686, t72, t14239, t22332, t10023, t22863, t14141, t23037, t22336, t13790, t6843);
        let (t86415, t86441, t86445, t86455, t86468) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1584(t10022, t2782, t86413, t1882, t6888, t22857, t555, t22953, t22954, t4101, t686, t72);
    (t86374, t86377, t86381, t86401, t86411, t86415, t86441, t86445, t86455, t86468)
}
