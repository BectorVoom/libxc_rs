//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1624;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1625;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta431(t3568: f64, t3588: f64, t12640: f64, t1284: f64, t3624: f64, t127: f64, t12866: f64, t3630: f64, t3712: f64, t12809: f64, t12811: f64, t12916: f64, t12732: f64, t3153: f64, t12952: f64, t3172: f64, t3711: f64, t1042: f64, t1250: f64, t12787: f64, t12910: f64, t12912: f64, t13081: f64, t17235: f64, t17261: f64, t17729: f64, t17730: f64, t2258: f64, t3362: f64, t3718: f64, t3720: f64, t44205: f64, t44599: f64, t44607: f64, t44609: f64, t44610: f64, t44616: f64, t5331: f64, t5333: f64, t5340: f64, t5341: f64, t12901: f64, t13033: f64, t13042: f64, t13047: f64, t3555: f64, t3781: f64, t5330: f64, t12861: f64, t11262: f64, t3600: f64, t3605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44618, t44624, t44634, t44637) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1624(t3568, t3588, t12640, t1284, t3624, t127, t12866, t3630, t3712, t12809, t12811, t12916);
        let (t44639, t44657) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1625(t12732, t3153, t12952, t3172, t3711, t1042, t1250, t12787, t12910, t12912, t13081, t17235, t17261, t17729, t17730, t2258, t3362, t3718, t3720, t44205, t44599, t44607, t44609, t44610, t44616, t44618, t44624, t44634, t44637, t5331, t5333, t5340, t5341);
        let (t44658, t44661, t44664, t44672, t44675) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1626(t12901, t13033, t13042, t13047, t3172, t3555, t3781, t5330, t12861, t12916, t3718, t11262, t3600, t3605);
    (t44618, t44639, t44657, t44658, t44661, t44664, t44672, t44675)
}
