//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1259/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1259<F: Float>(t23000: F, t33308: F, t7805: F, t28279: F, t3040: F, t28435: F, t28811: F, t7700: F, t8793: F, t7703: F, t13045: F, t22238: F, t787: F) -> (F, F, F, F, F, F, F) {
    let t33310 = t23000 * t33308 * t7805;
    let t33311 = F::cast_from(0.11502877786176224903e1_f64) * t33310;
    let t33313 = F::cast_from(0.71500979903700853338e0_f64) * t28279 * t3040;
    let t33315 = F::cast_from(0.35750489951850426669e0_f64) * t28435 * t3040;
    let t33317 = F::cast_from(0.71500979903700853338e0_f64) * t28811 * t3040;
    let t33319 = F::cast_from(0.21450293971110256002e1_f64) * t8793 * t7700;
    let t33321 = F::cast_from(0.10725146985555128001e1_f64) * t8793 * t7703;
    let t33325 = F::cast_from(0.53625734927775640005e1_f64) * t787 * t22238 * t13045;
    (t33311, t33313, t33315, t33317, t33319, t33321, t33325)
}
