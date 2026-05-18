//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 698/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk698<F: Float>(t12514: F, t2887: F, t12513: F, t12: F, t3: F, t213: F, t12476: F, t2872: F, t12485: F, t824: F, t12489: F, t12491: F, t12493: F, t5680: F, t5744: F) -> (F, F, F, F, F) {
    let t12515 = t12514 * t2887;
    let t12517 = F::new(0.96490945932906628932e2) * t12513 * t12515;
    let t12522 = F::new(1.0)/pow_3_2::<f64>(t12);
    let t12523 = t12522 * t3;
    let t12524 = t12523 * t213;
    let t12526 = t2872 * t12476;
    let t12528 = t824 * t12485;
    let t12531 = -F::new(0.25319e1) * t12489 + F::new(0.16879333333333333333e1) * t12491 - F::new(0.19692555555555555555e1) * t12493 - F::new(0.93011851851851851854e0) * t5680 + F::new(0.13651666666666666667e0) * t12524 - F::new(0.27303333333333333333e0) * t12526 - F::new(0.3185388888888888889e0) * t12528 - F::new(0.36514074074074074075e0) * t5744;
    (t12517, t12524, t12526, t12528, t12531)
}
