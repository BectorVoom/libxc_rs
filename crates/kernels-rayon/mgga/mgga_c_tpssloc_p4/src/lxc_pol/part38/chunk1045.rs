//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1045/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1045(t40: f64, t12943: f64, t1409: f64, t2517: f64, t707: f64, t3966: f64, t75: f64, t12606: f64, t1430: f64, t2244: f64, t2250: f64, t4104: f64, t607: f64, t767: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t12944 = 0.11696447245269292414e1_f64 * t12943;
    let t12945 = t2517 * t1409;
    let t12946 = t707 * t12945;
    let t12947 = 4.0_f64 * t12946;
    let t12950 = t75 * t3966;
    let t12958 = piecewise3(t146, 0.0_f64, 8.0_f64 / 27.0_f64 * t1430 * t2244 - 4.0_f64 / 9.0_f64 * t12950 * t607 - 2.0_f64 / 9.0_f64 * t4104 * t2250 + 2.0_f64 / 3.0_f64 * t767 * t12606);
    (t12944, t12947, t12958)
}
