//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1386/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1386(t34378: f64, t6717: f64, t6914: f64, t10532: f64, t10533: f64, t2487: f64, t6711: f64, t2898: f64, t6625: f64, t10444: f64, t1407: f64, t34363: f64, t587: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34573 = 0.37959496694381542179e3_f64 * t6914 * t6717 * t34378;
    let t34576 = 0.38649669361552115674e3_f64 * t10532 * t10533 * t34378;
    let t34579 = 0.14953741122029092374e3_f64 * t2487 * t6711 * t34378;
    let t34580 = t2898 * t6625;
    let t34581 = 0.89376224879626066674e-1_f64 * t34580;
    let t34582 = t1407 * t10444;
    let t34583 = 0.38342925953920749676e0_f64 * t34582;
    let t34585 = t587 * t912 * t34363;
    (t34573, t34576, t34579, t34581, t34583, t34585)
}
