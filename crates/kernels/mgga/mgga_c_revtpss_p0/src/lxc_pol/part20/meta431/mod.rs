//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1624;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1625;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta431<F: Float>(t3568: F, t3588: F, t12640: F, t1284: F, t3624: F, t127: F, t12866: F, t3630: F, t3712: F, t12809: F, t12811: F, t12916: F, t12732: F, t3153: F, t12952: F, t3172: F, t3711: F, t1042: F, t1250: F, t12787: F, t12910: F, t12912: F, t13081: F, t17235: F, t17261: F, t17729: F, t17730: F, t2258: F, t3362: F, t3718: F, t3720: F, t44205: F, t44599: F, t44607: F, t44609: F, t44610: F, t44616: F, t5331: F, t5333: F, t5340: F, t5341: F, t12901: F, t13033: F, t13042: F, t13047: F, t3555: F, t3781: F, t5330: F, t12861: F, t11262: F, t3600: F, t3605: F) -> (F, F, F, F, F, F, F, F) {
        let (t44618, t44624, t44634, t44637) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1624::<F>(t3568, t3588, t12640, t1284, t3624, t127, t12866, t3630, t3712, t12809, t12811, t12916);
        let (t44639, t44657) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1625::<F>(t12732, t3153, t12952, t3172, t3711, t1042, t1250, t12787, t12910, t12912, t13081, t17235, t17261, t17729, t17730, t2258, t3362, t3718, t3720, t44205, t44599, t44607, t44609, t44610, t44616, t44618, t44624, t44634, t44637, t5331, t5333, t5340, t5341);
        let (t44658, t44661, t44664, t44672, t44675) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1626::<F>(t12901, t13033, t13042, t13047, t3172, t3555, t3781, t5330, t12861, t12916, t3718, t11262, t3600, t3605);
    (t44618, t44639, t44657, t44658, t44661, t44664, t44672, t44675)
}
