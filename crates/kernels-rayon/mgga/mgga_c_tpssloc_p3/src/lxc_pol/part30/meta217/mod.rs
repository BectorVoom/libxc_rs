//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1014;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta217(t5726: f64, t913: f64, t893: f64, t2844: f64, t5694: f64, t2842: f64, t2848: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64, t1568: f64, t932: f64, t2868: f64, t2875: f64, t4384: f64, t5699: f64, t5706: f64, t5712: f64, t5714: f64, t5718: f64, t5721: f64, t5724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5727, t5729, t5730, t5732, t5737, t5742) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1014(t5726, t913, t893, t2844, t5694, t2842, t2848, t4335, t5679, t5683, t5687, t1568);
        let (t5743, t5758) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1015(t5742, t932, t2868, t2875, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
    (t5727, t5729, t5730, t5732, t5737, t5742, t5743, t5758)
}
