//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 860/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk860(t41191: f64, t41247: f64, t41257: f64, t41265: f64, t41355: f64, t41363: f64, t41365: f64, t40681: f64, t41767: f64, t42057: f64, t40944: f64, t40949: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43528 = 0.19513579069703984327e0_f64 * t41191;
    let t43558 = 0.77886770749688743854e-2_f64 * t41247;
    let t43566 = 0.2927036860455597649e0_f64 * t41257;
    let t43571 = 0.64905642291407286545e-2_f64 * t41265;
    let t43615 = 0.88895193539762595267e-1_f64 * t41355;
    let t43622 = 0.66671395154821946449e-1_f64 * t41363;
    let t43623 = 0.17740875559651324989e-2_f64 * t41365;
    let t43677 = 0.66211599834018861287e-4_f64 * t40681;
    let t43839 = 0.66211599834018861287e-4_f64 * t41767;
    let t43990 = 0.87811105813667929469e0_f64 * t42057;
    let t44083 = 0.58540737209111952978e0_f64 * t40944;
    let t44085 = 0.87811105813667929469e0_f64 * t40949;
    (t43528, t43558, t43566, t43571, t43615, t43622, t43623, t43677, t43839, t43990, t44083, t44085)
}
