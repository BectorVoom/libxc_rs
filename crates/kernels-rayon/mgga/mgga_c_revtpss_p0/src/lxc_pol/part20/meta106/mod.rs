//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk603;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk604;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk605;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk606;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk607;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk608;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk609;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta106(t2986: f64, t315: f64, t972: f64, t973: f64, t2846: f64, t2904: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2898: f64, t2900: f64, t2906: f64, t2910: f64, t2913: f64, t2916: f64, t963: f64, t323: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t2935: f64, t2938: f64, t2943: f64, t2945: f64, t2963: f64, t2968: f64, t2971: f64, t2980: f64, t2982: f64, t311: f64, t946: f64, t955: f64, t965: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2987, t2988) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk603(t2986, t315, t972);
        let (t2989, t3006) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk604(t2988, t973, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
        let t3007 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk605(t3006, t973);
        let t3010 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk606(t963);
        let t3011 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk607(t3010);
        let (t3012, t3013, t3014) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk608(t3011, t315, t323);
        let t3015 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk609(t2988, t3014);
        let t3018 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk610(t2868, t2871, t2878, t2921, t2929, t2935, t2938, t2943, t2945, t2963, t2968, t2971, t2980, t2982, t2987, t2989, t3007, t3012, t3015, t311, t946, t955, t965, t974);
    (t2987, t2988, t2989, t3006, t3007, t3010, t3011, t3012, t3013, t3014, t3015, t3018)
}
