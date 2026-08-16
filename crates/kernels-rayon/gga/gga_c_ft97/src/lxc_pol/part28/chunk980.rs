//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 980/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk980(t136891: f64, t5821: f64, t136898: f64, t136992: f64, t7335: f64, t136986: f64, t136457: f64, t32806: f64, t138873: f64, t542: f64, t137007: f64, t8811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t138894 = 0.20139801475612389137e-1_f64 * t5821 * t136891;
    let t138899 = t5821 * t136898;
    let t138924 = t7335 * t136992;
    let t138927 = 0.8891911659407557944e-2_f64 * t7335 * t136986;
    let t138930 = t32806 * t136457;
    let t138961 = t542 * t138873;
    let t138968 = t8811 * t137007;
    (t138894, t138899, t138924, t138927, t138930, t138961, t138968)
}
