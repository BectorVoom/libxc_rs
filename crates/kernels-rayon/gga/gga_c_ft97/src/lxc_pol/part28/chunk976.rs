//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 976/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk976(t1349: f64, t33001: f64, t376: f64, t136304: f64, t23701: f64, t23823: f64, t7203: f64, t2001: f64, t32772: f64, t3392: f64, t23711: f64, t173: f64, t32837: f64, t7195: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t138715 = t1349 * t376 * t33001;
    let t138725 = t23701 * t136304;
    let t138738 = t23823 * t7203;
    let t138739 = t2001 * t138738;
    let t138746 = t3392 * t32772 * t7203;
    let t138761 = t23711 * t136304;
    let t138769 = t7195 * t173 * t32837;
    (t138715, t138725, t138738, t138739, t138746, t138761, t138769)
}
