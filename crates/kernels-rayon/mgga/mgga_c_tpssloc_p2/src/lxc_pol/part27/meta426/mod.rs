//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1740;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta426(t22666: f64, t6907: f64, t1985: f64, t225: f64, t6956: f64, t562: f64, t794: f64, t6897: f64, t12030: f64, t12444: f64, t1375: f64, t1386: f64, t2016: f64, t22622: f64, t22624: f64, t22630: f64, t22639: f64, t22646: f64, t22650: f64, t22653: f64, t22656: f64, t22664: f64, t3882: f64, t3912: f64, t568: f64, t6958: f64, t6963: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t22667, t22668, t22670, t22674) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1740(t22666, t6907, t1985, t225, t6956, t562, t794);
        let (t22675, t22676, t22680) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1741(t22674, t6907, t6897, t12030, t12444, t1375, t1386, t2016, t22622, t22624, t22630, t22639, t22646, t22650, t22653, t22656, t22664, t22668, t22670, t3882, t3912, t568, t6958, t6963, t6993);
    (t22667, t22670, t22674, t22675, t22676, t22680)
}
