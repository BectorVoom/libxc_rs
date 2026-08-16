//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 698/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk698(t12514: f64, t2887: f64, t12513: f64, t12: f64, t3: f64, t213: f64, t12476: f64, t2872: f64, t12485: f64, t824: f64, t12489: f64, t12491: f64, t12493: f64, t5680: f64, t5744: f64) -> (f64, f64, f64, f64, f64) {
    let t12515 = t12514 * t2887;
    let t12517 = 0.96490945932906628932e2_f64 * t12513 * t12515;
    let t12522 = 1.0_f64/pow_3_2(t12);
    let t12523 = t12522 * t3;
    let t12524 = t12523 * t213;
    let t12526 = t2872 * t12476;
    let t12528 = t824 * t12485;
    let t12531 = -0.25319e1_f64 * t12489 + 0.16879333333333333333e1_f64 * t12491 - 0.19692555555555555555e1_f64 * t12493 - 0.93011851851851851854e0_f64 * t5680 + 0.13651666666666666667e0_f64 * t12524 - 0.27303333333333333333e0_f64 * t12526 - 0.3185388888888888889e0_f64 * t12528 - 0.36514074074074074075e0_f64 * t5744;
    (t12517, t12524, t12526, t12528, t12531)
}
