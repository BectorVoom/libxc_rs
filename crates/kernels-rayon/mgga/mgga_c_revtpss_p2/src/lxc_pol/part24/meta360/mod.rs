//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1229;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta360(t1089: f64, t23992: f64, t23997: f64, t24007: f64, t3304: f64, t3318: f64, t5004: f64, t6244: f64, t1082: f64, t24031: f64, t24111: f64, t23598: f64, t24042: f64, t380: f64, t6258: f64, t1024: f64, t11940: f64, t12122: f64, t12127: f64, t1647: f64, t16502: f64, t16544: f64, t16584: f64, t1689: f64, t1692: f64, t19566: f64, t23959: f64, t3204: f64, t3287: f64, t3317: f64, t342: f64, t381: f64, t4857: f64, t6235: f64, t6365: f64, t6368: f64, t6386: f64, t6389: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1229(t1089, t23992, t23997, t24007, t3304, t3318, t5004, t6244, t1082, t24031, t24111, t23598);
        let (t24162, t24167, t24176) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1230(t24042, t380, t5004, t6258, t1024, t11940, t12122, t12127, t1647, t16502, t16544, t16584, t1689, t1692, t19566, t23959, t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157, t3204, t3287, t3317, t342, t381, t4857, t6235, t6365, t6368, t6386, t6389);
    (t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157, t24162, t24167, t24176)
}
