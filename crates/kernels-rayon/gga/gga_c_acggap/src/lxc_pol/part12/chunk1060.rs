//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1060/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1060(t1554: f64, t30540: f64, t1558: f64, t4695: f64, t7822: f64, t4335: f64, t2068: f64, t4680: f64, t8521: f64, t30137: f64, t7585: f64, t8525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34857 = t7822 * t4695;
    let t34859 = t7822 * t4335;
    let t34862 = t2068 * t4680 * t8521;
    let t34865 = t7585 * t30137 * t8525;
    (t34853, t34855, t34857, t34859, t34862, t34865)
}
