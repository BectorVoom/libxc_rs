//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 641/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk641(t321: f64, t8946: f64, t333: f64, t118: f64, t4669: f64, t5155: f64, t5266: f64, t7793: f64, t7795: f64, t7815: f64, t7819: f64, t7821: f64, t8801: f64, t8804: f64, t8926: f64, t8933: f64, t8937: f64, t8940: f64, t8941: f64, t8944: f64) -> f64 {
    let t8947 = t8946 * t321;
    let t8950 = t8946 * t333;
    let t8955 = 0.34093327067806677161e-2_f64 * t8926 + 0.39914139006212695213e-1_f64 * t7793 + 0.11974241701863808564e0_f64 * t7795 - 0.79828278012425390426e-1_f64 * t7815 + t7819 - t7821 - 0.39914139006212695214e-1_f64 * t118 * t8804 - 0.39914139006212695214e-1_f64 * t118 * t8933 + 0.11974241701863808564e0_f64 * t5266 * t8937 + 0.11974241701863808564e0_f64 * t8940 * t8941 - 0.14967802127329760705e-1_f64 * t8944 - 0.17961362552795712846e0_f64 * t4669 * t8947 + 0.23948483403727617128e0_f64 * t5155 * t8950 - 0.39914139006212695214e-1_f64 * t118 * t8801;
    t8955
}
