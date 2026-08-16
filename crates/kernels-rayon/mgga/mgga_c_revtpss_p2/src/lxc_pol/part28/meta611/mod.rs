//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2133;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2134;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2135;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta611(t1937: f64, t98487: f64, t27123: f64, t6993: f64, t25803: f64, t7898: f64, t2033: f64, t47672: f64, t1907: f64, t4144: f64, t28196: f64, t27833: f64, t7313: f64, t1931: f64, t2371: f64, t13426: f64, t13544: f64, t1519: f64, t18153: f64, t18163: f64, t1932: f64, t2372: f64, t25805: f64, t27145: f64, t28025: f64, t28030: f64, t4254: f64, t4257: f64, t4293: f64, t6985: f64, t7007: f64, t7746: f64, t98472: f64, t98474: f64, t98477: f64, t98483: f64, t98486: f64, t3829: f64, t28167: f64, t8717: f64, t25082: f64, t28197: f64, t73488: f64, t13625: f64, t33651: f64, t25090: f64, t28187: f64, t7235: f64, t7003: f64, t13514: f64, t94: f64, t27126: f64, t25178: f64, t22496: f64, t32113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98489, t98491, t98494, t98499, t98501) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2133(t1937, t98487, t27123, t6993, t25803, t7898, t2033, t47672, t1907, t4144, t28196, t27833, t7313);
        let (t98507, t98512) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2134(t1931, t2371, t13426, t13544, t1519, t18153, t18163, t1932, t2372, t25805, t27145, t28025, t28030, t4254, t4257, t4293, t6985, t7007, t7746, t98472, t98474, t98477, t98483, t98486, t98489, t98491, t98494, t98499, t98501);
        let (t98522, t98525, t98528, t98530, t98532) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2135(t1907, t3829, t28167, t8717, t25082, t28197, t73488, t13625, t33651, t25090, t7898, t28187, t7235);
        let (t98534, t98537, t98539, t98541, t98544) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2136(t27123, t7003, t13514, t94, t1937, t27126, t6993, t25178, t7898, t22496, t25082, t32113);
    (t98507, t98512, t98522, t98525, t98528, t98530, t98532, t98534, t98537, t98539, t98541, t98544)
}
