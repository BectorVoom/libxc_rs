//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1339/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1339<F: Float>(t22142: F, t5498: F, t1102: F, t11632: F, t11640: F, t16545: F, t16547: F, t16562: F, t16567: F, t16587: F, t1924: F, t22091: F, t22095: F, t22099: F, t22103: F, t22107: F, t22111: F, t22116: F, t22120: F, t22128: F, t22131: F, t22135: F, t22139: F, t344: F, t4587: F, t5623: F) -> F {
    let t22143 = t5498 * t22142;
    let t22146 = -t16545 - t16547 + F::cast_from(0.73004774074074074073e-3_f64) * t22091 - F::cast_from(0.1478346675e-2_f64) * t1102 * t22095 + F::cast_from(0.19711289e-2_f64) * t1102 * t22099 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t22103 + F::cast_from(0.26281718666666666666e-2_f64) * t4587 * t22107 - F::cast_from(0.19711289e-2_f64) * t1102 * t22111 + F::cast_from(0.59133867e-2_f64) * t1102 * t22116 - F::cast_from(0.19711289e-2_f64) * t11632 * t22120 - F::cast_from(0.14600954814814814815e-3_f64) * t11640 + t16562 + t16567 - F::cast_from(0.87605728888888888887e-3_f64) * t16587 - F::cast_from(8.0_f64) * t1924 * t5623 + F::cast_from(0.1478346675e-2_f64) * t344 * t22128 - F::cast_from(0.19711289e-2_f64) * t22131 + F::cast_from(0.295669335e-2_f64) * t1102 * t22135 - F::cast_from(0.59133867e-2_f64) * t1102 * t22139 + F::cast_from(0.39422578e-2_f64) * t1102 * t22143;
    t22146
}
