//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2133;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2134;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2135;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta611<F: Float>(t1937: F, t98487: F, t27123: F, t6993: F, t25803: F, t7898: F, t2033: F, t47672: F, t1907: F, t4144: F, t28196: F, t27833: F, t7313: F, t1931: F, t2371: F, t13426: F, t13544: F, t1519: F, t18153: F, t18163: F, t1932: F, t2372: F, t25805: F, t27145: F, t28025: F, t28030: F, t4254: F, t4257: F, t4293: F, t6985: F, t7007: F, t7746: F, t98472: F, t98474: F, t98477: F, t98483: F, t98486: F, t3829: F, t28167: F, t8717: F, t25082: F, t28197: F, t73488: F, t13625: F, t33651: F, t25090: F, t28187: F, t7235: F, t7003: F, t13514: F, t94: F, t27126: F, t25178: F, t22496: F, t32113: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98489, t98491, t98494, t98499, t98501) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2133::<F>(t1937, t98487, t27123, t6993, t25803, t7898, t2033, t47672, t1907, t4144, t28196, t27833, t7313);
        let (t98507, t98512) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2134::<F>(t1931, t2371, t13426, t13544, t1519, t18153, t18163, t1932, t2372, t25805, t27145, t28025, t28030, t4254, t4257, t4293, t6985, t7007, t7746, t98472, t98474, t98477, t98483, t98486, t98489, t98491, t98494, t98499, t98501);
        let (t98522, t98525, t98528, t98530, t98532) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2135::<F>(t1907, t3829, t28167, t8717, t25082, t28197, t73488, t13625, t33651, t25090, t7898, t28187, t7235);
        let (t98534, t98537, t98539, t98541, t98544) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2136::<F>(t27123, t7003, t13514, t94, t1937, t27126, t6993, t25178, t7898, t22496, t25082, t32113);
    (t98507, t98512, t98522, t98525, t98528, t98530, t98532, t98534, t98537, t98539, t98541, t98544)
}
