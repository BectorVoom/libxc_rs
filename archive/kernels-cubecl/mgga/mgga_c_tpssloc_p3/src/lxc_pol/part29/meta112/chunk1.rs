//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 687/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk687<F: Float>(t40: F, t52: F, t2535: F, t761: F, t718: F, t751: F, t2244: F, t2250: F, t75: F, t767: F, t771: F, t78: F, zeta_threshold: F) -> (F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t2537 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t2535;
    let t2538 = t718 * t751;
    let t2539 = F::cast_from(2.0_f64) * t2538;
    let t2545 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t75 * t2244 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t2250);
    let t2551 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t78 * t2244 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t2250);
    let t2553 = t2545 / F::cast_from(2.0_f64) + t2551 / F::cast_from(2.0_f64);
    (t2537, t2538, t2539, t2553)
}
