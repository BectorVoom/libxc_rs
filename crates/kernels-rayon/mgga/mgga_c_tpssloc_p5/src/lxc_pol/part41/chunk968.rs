//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 968/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk968(t12858: f64, t763: f64, t1472: f64, t2517: f64, t4303: f64, t870: f64, t2430: f64, t4205: f64, t1409: f64, t750: f64, t607: f64, t4194: f64) -> (f64, f64, f64, f64, f64) {
    let t12860 = 0.11696447245269292414e1_f64 * t12858 * t763;
    let t12861 = t1472 * t2517;
    let t12895 = t4303 * t870;
    let t12922 = 8.0_f64 * t4205 * t2430;
    let t12923 = t750 * t1409;
    let t12924 = t12923 * t607;
    let t12926 = 24.0_f64 * t4194 * t12924;
    (t12860, t12861, t12895, t12922, t12926)
}
