//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2635;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta669(t20721: f64, t247: f64, t3719: f64, t3670: f64, t5390: f64, t1225: f64, t18281: f64, t1012: f64, t1010: f64, t5843: f64, t5378: f64, t5381: f64, t21040: f64, t3629: f64, t3626: f64, t12840: f64, t20795: f64, t1222: f64, t1227: f64, t13012: f64, t17593: f64, t17619: f64, t17622: f64, t3625: f64, t5340: f64, t5369: f64, t5373: f64, t5384: f64, t5386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21200, t21203) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2634(t20721, t247, t3719, t3670, t5390);
        let (t21209, t21210, t21213, t21216, t21218, t21219, t21222) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2635(t1225, t18281, t1012, t1010, t5843, t5378, t5381, t21040, t3629, t3626, t12840, t20795);
        let (t21223, t21226) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2636(t21222, t3626, t1222, t1227, t13012, t17593, t17619, t17622, t21200, t21203, t21210, t21213, t21216, t21219, t3625, t5340, t5369, t5373, t5384, t5386);
    (t21200, t21203, t21209, t21213, t21218, t21219, t21222, t21223, t21226)
}
