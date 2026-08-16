//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1142/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1142<F: Float>(t10306: F, t1685: F, t2474: F, t4041: F, t42034: F, t43956: F, t43957: F, t43978: F, t43979: F, t47757: F, t47759: F, t47765: F, t47767: F, t47772: F, t47774: F, t47785: F, t47787: F, t47795: F, t47797: F, t72: F) -> F {
    let t49686 = -F::cast_from(0.1702583995731913576e-4_f64) * t47757 + t43956 - t43957 - F::cast_from(0.23948483403727617128e0_f64) * t4041 * t10306 + F::cast_from(0.36366215538993788973e-1_f64) * t47759 + F::cast_from(0.3192344991997337955e-4_f64) * t47765 + F::cast_from(0.212822999466489197e-4_f64) * t47767 + F::cast_from(0.212822999466489197e-4_f64) * t47772 + F::cast_from(0.95793933614910468512e0_f64) * t47774 + F::cast_from(2.0_f64) * t72 * t1685 * t2474 + F::cast_from(0.81823984962736025192e-1_f64) * t47785 - F::cast_from(0.16364796992547205038e0_f64) * t47787 - t43978 - t43979 - F::cast_from(0.11974241701863808564e0_f64) * t47795 - F::cast_from(0.47896966807455234255e0_f64) * t42034 + F::cast_from(0.17961362552795712846e0_f64) * t47797;
    t49686
}
