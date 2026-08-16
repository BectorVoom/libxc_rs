//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 812/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk812<F: Float>(t12514: F, t2887: F, t12513: F, t12: F, t3: F, t213: F, t12476: F, t2872: F, t12485: F, t824: F, t12489: F, t12491: F, t12493: F, t5680: F, t5744: F) -> (F, F, F, F, F) {
    let t12515 = t12514 * t2887;
    let t12517 = F::cast_from(0.96490945932906628932e2_f64) * t12513 * t12515;
    let t12522 = F::cast_from(1.0_f64)/pow_3_2::<F>(t12);
    let t12523 = t12522 * t3;
    let t12524 = t12523 * t213;
    let t12526 = t2872 * t12476;
    let t12528 = t824 * t12485;
    let t12531 = -F::cast_from(0.25319e1_f64) * t12489 + F::cast_from(0.16879333333333333333e1_f64) * t12491 - F::cast_from(0.19692555555555555555e1_f64) * t12493 - F::cast_from(0.93011851851851851854e0_f64) * t5680 + F::cast_from(0.13651666666666666667e0_f64) * t12524 - F::cast_from(0.27303333333333333333e0_f64) * t12526 - F::cast_from(0.3185388888888888889e0_f64) * t12528 - F::cast_from(0.36514074074074074075e0_f64) * t5744;
    (t12517, t12524, t12526, t12528, t12531)
}
