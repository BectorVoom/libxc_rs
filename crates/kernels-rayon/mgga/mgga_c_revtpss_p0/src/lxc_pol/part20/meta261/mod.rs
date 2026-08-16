//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1104;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1105;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1106;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta261(t11545: f64, t291: f64, t2942: f64, t941: f64, t11410: f64, t954: f64, t2986: f64, t960: f64, t11467: f64, t973: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64, t324: f64, t11291: f64, t11293: f64, t11296: f64, t11303: f64, t11382: f64, t11390: f64, t11521: f64, t11525: f64, t11530: f64, t11533: f64, t2945: f64, t2968: f64, t2987: f64, t2989: f64, t3012: f64, t311: f64, t11520: f64, t300: f64, t2979: f64, t983: f64, t11392: f64, t11394: f64, t11398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11547, t11548, t11551, t11554, t11557, t11571) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1104(t11545, t291, t2942, t941, t11410, t954, t2986, t960, t11467, t973, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let (t11572, t11585) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1105(t11571, t324, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let t11588 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1106(t11291, t11293, t11296, t11303, t11382, t11390, t11521, t11525, t11530, t11533, t11547, t11548, t11551, t11554, t11557, t11572, t11585, t2945, t2968, t2987, t2989, t3012, t311);
        let (t11590, t11591, t11593, t11594) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1107(t11520, t11588, t300, t2979, t983, t11291, t11293, t11296, t11303, t11382, t11390, t11392, t11394, t11398);
    (t11547, t11548, t11551, t11554, t11557, t11571, t11572, t11585, t11590, t11591, t11593, t11594)
}
