//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 687/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk687(t1640: f64, t4741: f64, t4745: f64, t4740: f64, t583: f64, t573: f64, t10560: f64, t4744: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10587: f64, t10595: f64, t10607: f64, t10610: f64, t10613: f64, t10615: f64, t10617: f64, t10619: f64, t10623: f64, t10626: f64) -> (f64, f64, f64) {
    let t10710 = t1640 * t4741;
    let t10712 = 0.48245472966453314466e2_f64 * t10710 * t4745;
    let t10714 = 1.0_f64 / t4740 / t583;
    let t10715 = t573 * t10714;
    let t10716 = t10560 * t4744;
    let t10718 = 0.96490945932906628932e2_f64 * t10715 * t10716;
    let t10733 = -0.32862666666666666666e0_f64 * t10607 + 0.16431333333333333333e0_f64 * t10610 - 0.49293999999999999999e0_f64 * t10613 - 0.27385555555555555556e0_f64 * t10615 + 0.16431333333333333333e0_f64 * t10617 + 0.5477111111111111111e-1_f64 * t10619 - 0.36514074074074074075e-1_f64 * t10623 - 0.82156666666666666667e-1_f64 * t10626 - 0.59793333333333333333e0_f64 * t10587 + 0.17938e1_f64 * t10595 - 0.39862222222222222223e0_f64 * t10570 + 0.19931111111111111111e0_f64 * t10572 - 0.59793333333333333333e0_f64 * t10574 + 0.29896666666666666667e0_f64 * t10576;
    (t10712, t10718, t10733)
}
