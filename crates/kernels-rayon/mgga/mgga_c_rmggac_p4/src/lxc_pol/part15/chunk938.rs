//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 938/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk938(t7691: f64, t9783: f64, t1835: f64, t1979: f64, t1982: f64, t201: f64, t457: f64, t2191: f64, t9932: f64, t39506: f64, t39529: f64, t39536: f64, t39545: f64, t39556: f64, t39558: f64, t45579: f64, t45584: f64, t45589: f64, t45591: f64, t45593: f64, t45595: f64, t45597: f64, t45599: f64, t45601: f64) -> f64 {
    let t45603 = t7691 * t9783;
    let t45608 = t1835 * t457 * t201 * t1979 * t1982;
    let t45610 = t2191 * t9932;
    let t45612 = -0.6818665413561335432e-1_f64 * t45579 - 0.1064114997332445985e-4_f64 * t45584 + 0.25538759935978703638e-4_f64 * t45589 - 0.4726e1_f64 * t45591 - 0.14967802127329760705e-1_f64 * t45593 - t39506 - 0.2993560425465952141e-1_f64 * t45595 + 0.19863479950205658386e-4_f64 * t45597 - 0.59590439850616975155e-4_f64 * t45599 - t39529 + t39536 + t39545 - 0.1064114997332445985e-4_f64 * t45601 - 0.53205749866622299248e-5_f64 * t45603 - t39556 - t39558 + 0.42564599893297839398e-5_f64 * t45608 + 0.85129199786595678796e-5_f64 * t45610;
    t45612
}
