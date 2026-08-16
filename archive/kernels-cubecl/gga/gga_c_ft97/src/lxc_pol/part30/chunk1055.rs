//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1055/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1055<F: Float>(t263: F, t35516: F, t35604: F, t41402: F, t24412: F, t27983: F, t13927: F, t33595: F, t13830: F, t7553: F, t10157: F, t1403: F, t141410: F, t141420: F, t141431: F, t141435: F, t193: F, t2354: F, t27894: F, t27943: F, t33502: F, t3837: F, t4003: F, t6002: F, t684: F, t7437: F, t7441: F, t7443: F) -> (F, F, F, F, F) {
    let t151066 = t35516 * t263;
    let t151077 = t41402 * t35604;
    let t151079 = t24412 * t27983;
    let t151081 = t13927 * t33595;
    let t151092 = t13830 * t7553;
    let t151094 = -t6002 * t2354 * t151066 * t684 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) * t6002 * t10157 * t33502 * t3837 + t141410 + t7437 * t27943 / F::cast_from(6.0_f64) - F::cast_from(12.0_f64) * t151077 + F::cast_from(8.0_f64) * t151079 + F::cast_from(4.0_f64) * t151081 - t1403 * t193 * t7441 * t4003 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t141420 + t141431 / F::cast_from(54.0_f64) - t27894 * t7443 / F::cast_from(3.0_f64) - t141435 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) * t151092;
    (t151077, t151079, t151081, t151092, t151094)
}
