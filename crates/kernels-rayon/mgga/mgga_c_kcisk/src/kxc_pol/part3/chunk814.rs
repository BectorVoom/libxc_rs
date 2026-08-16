//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 814/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk814(t12535: f64, t2918: f64, t848: f64, t2900: f64, t846: f64, t3346: f64, t12489: f64, t12491: f64, t12493: f64, t12524: f64, t12526: f64, t12528: f64, t5680: f64, t5744: f64) -> (f64, f64, f64) {
    let t12537 = t2918 * t12535 * t848;
    let t12540 = t2900 * t846;
    let t12541 = t12540 * t3346;
    let t12552 = -0.34523333333333333333e1_f64 * t12489 + 0.23015555555555555556e1_f64 * t12491 - 0.26851481481481481482e1_f64 * t12493 - 0.93932222222222222223e0_f64 * t5680 + 0.73355e-1_f64 * t12524 - 0.14671e0_f64 * t12526 - 0.17116166666666666667e0_f64 * t12528 - 0.36793333333333333333e0_f64 * t5744;
    (t12537, t12541, t12552)
}
