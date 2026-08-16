//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1142/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1142(t10306: f64, t1685: f64, t2474: f64, t4041: f64, t42034: f64, t43956: f64, t43957: f64, t43978: f64, t43979: f64, t47757: f64, t47759: f64, t47765: f64, t47767: f64, t47772: f64, t47774: f64, t47785: f64, t47787: f64, t47795: f64, t47797: f64, t72: f64) -> f64 {
    let t49686 = -0.1702583995731913576e-4_f64 * t47757 + t43956 - t43957 - 0.23948483403727617128e0_f64 * t4041 * t10306 + 0.36366215538993788973e-1_f64 * t47759 + 0.3192344991997337955e-4_f64 * t47765 + 0.212822999466489197e-4_f64 * t47767 + 0.212822999466489197e-4_f64 * t47772 + 0.95793933614910468512e0_f64 * t47774 + 2.0_f64 * t72 * t1685 * t2474 + 0.81823984962736025192e-1_f64 * t47785 - 0.16364796992547205038e0_f64 * t47787 - t43978 - t43979 - 0.11974241701863808564e0_f64 * t47795 - 0.47896966807455234255e0_f64 * t42034 + 0.17961362552795712846e0_f64 * t47797;
    t49686
}
