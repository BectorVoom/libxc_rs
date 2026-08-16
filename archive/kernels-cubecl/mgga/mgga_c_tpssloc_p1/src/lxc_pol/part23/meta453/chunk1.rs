//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1306/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1306<F: Float>(t52: F, t16649: F, t20217: F, t2298: F, t4111: F, t5398: F, t75836: F, t75847: F, t75912: F, t771: F, t78: F, t75964: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t75976 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t2298 * t75836 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t16649 * t5398 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t78 * t75847 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4111 * t20217 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t75912);
    let t75978 = t75964 / F::cast_from(2.0_f64) + t75976 / F::cast_from(2.0_f64);
    t75978
}
