//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3140/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3140(t16831: f64, t300: f64, t1198: f64, t56176: f64, t56183: f64, t43830: f64, t43832: f64, t45000: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64) -> (f64, f64) {
    let t57861 = t300 * t16831;
    let t57863 = 0.17544670867903938621e1_f64 * t57861 * t1198;
    let t57872 = 0.15829629629629629629e-1_f64 * t56176;
    let t57874 = 0.47488888888888888888e-1_f64 * t56183;
    let t57883 = t45000 - 0.42739999999999999999e0_f64 * t56151 + 0.10685e0_f64 * t56155 + 0.32055e0_f64 * t56159 + 0.35616666666666666666e-1_f64 * t56163 + 0.4274e0_f64 * t56167 - 0.35616666666666666666e-1_f64 * t43830 + 0.11872222222222222222e-1_f64 * t43832 - 0.52765432098765432099e-1_f64 * t56174 - t57872 + 0.23744444444444444444e0_f64 * t56181 + t57874 - 0.71233333333333333332e-1_f64 * t56185 - 0.35616666666666666666e-1_f64 * t56187 - 0.10685e0_f64 * t56189 - 0.35616666666666666666e-1_f64 * t56194 - 0.35616666666666666666e-1_f64 * t56198 - 0.2137e0_f64 * t56203 - 0.11872222222222222222e-1_f64 * t56207 + 0.23744444444444444444e-1_f64 * t56209;
    (t57863, t57883)
}
