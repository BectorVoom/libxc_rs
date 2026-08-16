//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2126/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2126(t10828: f64, t300: f64, t2930: f64, t3030: f64, t4552: f64, t3032: f64, t3129: f64, t42875: f64, t4338: f64, t973: f64, t13965: f64, t3114: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49532 = t300 * t10828;
    let t49541 = t300 * t2930;
    let t49649 = t4552 * t3030;
    let t49650 = t49649 * t3032;
    let t49651 = t49650 * t3129;
    let t49661 = t973 * t42875 * t4338;
    let t49662 = t49661 / 324.0_f64;
    let t49690 = t3114 * t13965;
    (t49532, t49541, t49649, t49650, t49651, t49662, t49690)
}
