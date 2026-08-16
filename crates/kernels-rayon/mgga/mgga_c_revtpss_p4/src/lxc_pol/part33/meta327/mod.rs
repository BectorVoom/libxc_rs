//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1334;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta327(t221: f64, t346: f64, t68: f64, t345: f64, t245: f64, t3089: f64, t3088: f64, t3114: f64, t11223: f64, t225: f64, t366: f64, t1026: f64, t371: f64, t676: f64, t1025: f64, t271: f64, t2857: f64, t283: f64, t3298: f64, t994: f64, t4891: f64, t3154: f64, t999: f64, t1086: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11737, t11772, t11773, t11774, t11788, t11789, t11817) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1334(t221, t346, t68, t345, t245, t3089, t3088, t3114, t11223, t225, t366, t1026, t371, t676);
        let (t11818, t11821, t11852, t11859, t11860, t11865) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1335(t1025, t11817, t271, t2857, t283, t3298, t994, t4891, t3154, t999, t1086, t3046);
    (t11737, t11772, t11773, t11774, t11788, t11789, t11818, t11821, t11852, t11859, t11860, t11865)
}
