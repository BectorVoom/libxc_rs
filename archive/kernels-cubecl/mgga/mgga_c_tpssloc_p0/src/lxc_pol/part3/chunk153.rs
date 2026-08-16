//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 153/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk153<F: Float>(t407: F, t410: F, t413: F, t417: F) -> (F, F, F) {
    let t445 = F::cast_from(0.51785e1_f64) * t410 + F::cast_from(0.905775e0_f64) * t407 + F::cast_from(0.1100325e0_f64) * t413 + F::cast_from(0.1241775e0_f64) * t417;
    let t448 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t445;
    let t449 = F::ln(t448);
    (t445, t448, t449)
}
