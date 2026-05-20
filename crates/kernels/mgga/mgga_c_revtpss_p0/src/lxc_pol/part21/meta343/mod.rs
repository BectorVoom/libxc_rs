//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1670;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1671;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1672;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta343<F: Float>(t11545: F, t291: F, t2942: F, t941: F, t11410: F, t954: F, t2986: F, t960: F, t11467: F, t973: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t324: F, t11291: F, t11293: F, t11296: F, t11303: F, t11382: F, t11390: F, t11521: F, t11525: F, t11530: F, t11533: F, t2945: F, t2968: F, t2987: F, t2989: F, t3012: F, t311: F, t11520: F, t300: F, t2979: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11547, t11548, t11551, t11554, t11557, t11560, t11571) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1670::<F>(t11545, t291, t2942, t941, t11410, t954, t2986, t960, t11467, t973, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let (t11572, t11574, t11585) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1671::<F>(t11571, t324, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let t11588 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1672::<F>(t11291, t11293, t11296, t11303, t11382, t11390, t11521, t11525, t11530, t11533, t11547, t11548, t11551, t11554, t11557, t11572, t11585, t2945, t2968, t2987, t2989, t3012, t311);
        let (t11590, t11591) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1673::<F>(t11520, t11588, t300, t2979);
    (t11547, t11548, t11551, t11554, t11557, t11560, t11571, t11572, t11574, t11585, t11590, t11591)
}
