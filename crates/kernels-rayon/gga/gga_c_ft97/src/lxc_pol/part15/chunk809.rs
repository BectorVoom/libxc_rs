//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 809/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk809(t21837: f64, t898: f64, t900: f64, t4357: f64, t5468: f64, t20489: f64, t231: f64, t893: f64, t1268: f64, t5457: f64, t10904: f64, t10915: f64, t10916: f64, t21181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21839 = t898 * t900 * t21837;
    let t21843 = t898 * t4357 * t5468;
    let t21847 = t231 * t893 * t20489;
    let t21850 = t5457 * t1268;
    let t21852 = t898 * t10904 * t21850;
    let t21856 = t10915 * t10916 * t21181;
    (t21839, t21843, t21847, t21850, t21852, t21856)
}
