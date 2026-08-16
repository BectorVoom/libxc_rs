//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 792/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk792(t3521: f64, t8900: f64, t8904: f64, t2063: f64, t2372: f64, t1417: f64, t8916: f64, t8932: f64, t8924: f64, t821: f64, t8511: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22412 = t3521 * t8900;
    let t22414 = t3521 * t8904;
    let t22417 = t2063 * t2372;
    let t22469 = t1417 * t8916;
    let t22512 = t1417 * t8932;
    let t22524 = t1417 * t8924;
    let t22564 = t821 * t8511;
    (t22412, t22414, t22417, t22469, t22512, t22524, t22564)
}
