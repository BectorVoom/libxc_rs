//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1317/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1317(t96217: f64, t27811: f64, t61287: f64, t4981: f64, t982: f64, t990: f64, t26757: f64, t27832: f64, t26748: f64, t27911: f64, t7706: f64, t93087: f64, t93425: f64, t95917: f64, t95923: f64, t96204: f64, t96207: f64, t96212: f64, t96215: f64) -> f64 {
    let t96218 = 0.22109259259259259258e-2_f64 * t96217;
    let t96221 = t27811 * t61287;
    let t96227 = t4981 * t982 * t990;
    let t96231 = 0.15445601851851851852e-3_f64 * t27832 * t26757;
    let t96232 = 0.29479012345679012345e-2_f64 * t96204 - 0.11054629629629629629e-2_f64 * t96207 - 0.16581944444444444444e-2_f64 * t93087 - 0.49745833333333333332e-2_f64 * t96212 + 0.33163888888888888888e-2_f64 * t96215 + t96218 - 0.61836467013888888888e-4_f64 * t93425 * t95917 - 0.12378114784505208333e-4_f64 * t96221 * t95923 - 0.13901041666666666667e-2_f64 * t26748 * t27911 + 0.12356481481481481482e-2_f64 * t96227 * t7706 - t96231;
    t96232
}
