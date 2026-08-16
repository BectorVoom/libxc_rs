//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1185/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1185(t11285: f64, t3377: f64, t14853: f64, t1164: f64, t300: f64, t4832: f64, t1166: f64, t3419: f64, t4869: f64, t11180: f64, t1671: f64, t3259: f64, t4782: f64) -> (f64, f64, f64, f64, f64) {
    let t14854 = t11285 * t3377;
    let t14855 = t14853 * t14854;
    let t14857 = 0.10254018858216406658e4_f64 * t1164 * t14855;
    let t14858 = t300 * t4832;
    let t14860 = 0.11696447245269292414e1_f64 * t14858 * t1166;
    let t14862 = 0.5848223622634646207e0_f64 * t4869 * t3419;
    let t14864 = 1.0_f64 * t11180 * t1671;
    let t14866 = 2.0_f64 * t3259 * t4782;
    (t14857, t14860, t14862, t14864, t14866)
}
