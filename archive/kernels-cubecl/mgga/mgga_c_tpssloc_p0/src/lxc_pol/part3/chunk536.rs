//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 536/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk536<F: Float>(t40: F, t52: F, t2427: F, t708: F, t607: F, t751: F, t707: F, t195: F, t2244: F, t2250: F, t73: F, t197: F, t76: F, t157: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t2429 = F::cast_from(8.0_f64) * t2427 * t708;
    let t2430 = t751 * t607;
    let t2431 = t707 * t2430;
    let t2432 = F::cast_from(8.0_f64) * t2431;
    let t2433 = F::cast_from(1.0_f64) / t195;
    let t2439 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2433 * t2244 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73 * t2250);
    let t2440 = F::cast_from(1.0_f64) / t197;
    let t2446 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2440 * t2244 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t2250);
    let t2447 = t2439 + t2446;
    let t2448 = t2447 * t157;
    (t2429, t2430, t2431, t2432, t2433, t2440, t2447, t2448)
}
