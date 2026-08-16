//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2004/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2004(t14536: f64, t225: f64, t14532: f64, t14562: f64, t14527: f64, t14534: f64, t16465: f64, t12250: f64, t1824: f64, t1799: f64, t3791: f64, t3850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50625 = t14536 * t225;
    let t50632 = t14532 * t225;
    let t50653 = t14562 * t225;
    let t50690 = t14527 * t225;
    let t50703 = t14534 * t225;
    let t53866 = t16465 * t225;
    let t54014 = t1824 * t12250;
    let t54068 = t1799 * t3791;
    let t54153 = t1824 * t3850;
    (t50625, t50632, t50653, t50690, t50703, t53866, t54014, t54068, t54153)
}
