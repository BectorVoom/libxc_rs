//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1964/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1964<F: Float>(t13042: F, t17064: F, t2054: F, t259: F, t26713: F, t4142: F, t4273: F, t59503: F, t7087: F, t7823: F, t7830: F, t86870: F, t92375: F, t92382: F, t92390: F, t92393: F, t98117: F, t98122: F, t98125: F, t98135: F, t98148: F, t98153: F, t98158: F, t98164: F, t98172: F, t98181: F) -> F {
    let t101335 = F::cast_from(0.15352717957250113407e0_f64) * t98117 - F::cast_from(0.9869604401089358619e-1_f64) * t98122 + F::cast_from(0.6579736267392905746e-1_f64) * t98125 + t92375 + F::cast_from(2.0_f64) * t4142 * t7823 * t259 + F::cast_from(4.0_f64) * t26713 * t4273 + t92382 - F::cast_from(0.20835831513410868196e0_f64) * t86870 - F::cast_from(0.16449340668482264365e-1_f64) * t98135 + t92390 + F::cast_from(0.3289868133696452873e-1_f64) * t98148 - F::cast_from(0.39478417604357434476e0_f64) * t98153 - F::cast_from(0.6579736267392905746e-1_f64) * t98158 + t92393 - t59503 * t2054 - F::cast_from(0.6579736267392905746e-1_f64) * t98164 - F::cast_from(6.0_f64) * t7087 * t17064 + F::cast_from(4.0_f64) * t13042 * t7830 + F::cast_from(0.9869604401089358619e-1_f64) * t98172 + F::cast_from(0.3289868133696452873e-1_f64) * t98181;
    t101335
}
