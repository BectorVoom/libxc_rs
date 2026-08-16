//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 891/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk891<F: Float>(t154: F, t3061: F, t10544: F, t276: F, t285: F, t273: F, t2928: F, t941: F, t2931: F, t323: F, t2841: F, t888: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10564 = t154 * t3061;
    let t10577 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10544;
    let t10595 = F::cast_from(1.0_f64) / t276 / t285 / F::cast_from(4.0_f64);
    let t10599 = F::cast_from(1.0_f64)/pow_3_2::<F>(t273);
    let t10608 = F::cast_from(0.28842592592592592592e-1_f64) * t10544;
    let t10629 = F::cast_from(1.0_f64) / t2928 / t941;
    let t10632 = F::cast_from(1.0_f64) / t2931 / t323;
    let t10636 = F::cast_from(0.55403703703703703703e-1_f64) * t10544;
    let t10655 = t888 * t2841;
    (t10564, t10577, t10595, t10599, t10608, t10629, t10632, t10636, t10655)
}
