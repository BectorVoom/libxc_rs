//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2039/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2039(t2332: f64, t81442: f64, t22470: f64, t2358: f64, t63: f64, t9365: f64, t193: f64, t201: f64, t6665: f64, t23285: f64, t2752: f64, t10143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81443 = t81442 * t2332;
    let t81445 = t22470 * t2358;
    let t81446 = t63 * t9365;
    let t81483 = t193 * t201 * t6665;
    let t81525 = t23285 * t2752;
    let t81539 = t6665 * t10143;
    (t81443, t81445, t81446, t81483, t81525, t81539)
}
