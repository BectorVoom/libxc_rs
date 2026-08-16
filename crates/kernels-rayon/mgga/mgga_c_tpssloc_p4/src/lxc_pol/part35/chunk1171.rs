//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1171/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1171(t24826: f64, t8074: f64, t3247: f64, t491: f64, t7359: f64, t7999: f64, t1222: f64, t8043: f64, t8049: f64, t5017: f64, t7337: f64, t1207: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27556 = t24826 * t8074;
    let t27561 = t491 * t3247;
    let t27572 = t7999 * t7359;
    let t27578 = t8043 * t1222;
    let t27592 = t8049 * t1222;
    let t27598 = t7337 * t5017;
    let t27599 = t1207 * t27598;
    (t27556, t27561, t27572, t27578, t27592, t27598, t27599)
}
