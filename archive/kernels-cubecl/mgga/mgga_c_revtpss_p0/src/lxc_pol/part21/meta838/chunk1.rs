//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3140/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3140<F: Float>(t16831: F, t300: F, t1198: F, t56176: F, t56183: F, t43830: F, t43832: F, t45000: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F) -> (F, F) {
    let t57861 = t300 * t16831;
    let t57863 = F::cast_from(0.17544670867903938621e1_f64) * t57861 * t1198;
    let t57872 = F::cast_from(0.15829629629629629629e-1_f64) * t56176;
    let t57874 = F::cast_from(0.47488888888888888888e-1_f64) * t56183;
    let t57883 = t45000 - F::cast_from(0.42739999999999999999e0_f64) * t56151 + F::cast_from(0.10685e0_f64) * t56155 + F::cast_from(0.32055e0_f64) * t56159 + F::cast_from(0.35616666666666666666e-1_f64) * t56163 + F::cast_from(0.4274e0_f64) * t56167 - F::cast_from(0.35616666666666666666e-1_f64) * t43830 + F::cast_from(0.11872222222222222222e-1_f64) * t43832 - F::cast_from(0.52765432098765432099e-1_f64) * t56174 - t57872 + F::cast_from(0.23744444444444444444e0_f64) * t56181 + t57874 - F::cast_from(0.71233333333333333332e-1_f64) * t56185 - F::cast_from(0.35616666666666666666e-1_f64) * t56187 - F::cast_from(0.10685e0_f64) * t56189 - F::cast_from(0.35616666666666666666e-1_f64) * t56194 - F::cast_from(0.35616666666666666666e-1_f64) * t56198 - F::cast_from(0.2137e0_f64) * t56203 - F::cast_from(0.11872222222222222222e-1_f64) * t56207 + F::cast_from(0.23744444444444444444e-1_f64) * t56209;
    (t57863, t57883)
}
