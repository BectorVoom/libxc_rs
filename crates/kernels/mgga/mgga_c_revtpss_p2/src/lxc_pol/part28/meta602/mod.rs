//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2079;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2080;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta602<F: Float>(t26093: F, t575: F, t116: F, t25832: F, t26133: F, t571: F, t2327: F, t7724: F, t27833: F, t7316: F, t13426: F, t7003: F, t18227: F, t25861: F, t4248: F, t3813: F, t651: F, t7741: F, t28159: F, t18153: F, t1936: F, t670: F, t6982: F, t13429: F, t13521: F, t13532: F, t13540: F, t1519: F, t2007: F, t2320: F, t2328: F, t2331: F, t25805: F, t27830: F, t28030: F, t4297: F, t508: F, t649: F, t671: F, t6985: F, t7883: F, t92737: F) -> (F, F, F, F, F, F, F) {
        let (t95127, t95137, t95180, t97593, t97604, t97606) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2079::<F>(t26093, t575, t116, t25832, t26133, t571, t2327, t7724, t27833, t7316, t13426, t7003);
        let (t97608, t97610, t97617, t97622, t97629, t97632) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2080::<F>(t18227, t7003, t25861, t4248, t3813, t651, t7741, t116, t28159, t18153, t1936, t670, t6982);
        let t97635 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2081::<F>(t13429, t13521, t13532, t13540, t1519, t2007, t2320, t2328, t2331, t25805, t27830, t28030, t4297, t508, t649, t671, t6985, t7883, t92737, t97593, t97604, t97606, t97608, t97610, t97617, t97622, t97629, t97632);
    (t95127, t95137, t95180, t97593, t97622, t97632, t97635)
}
