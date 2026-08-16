//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2060/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2060(t23384: f64, t25827: f64, t25436: f64, t23328: f64, t23394: f64, t1054: f64, t4693: f64, t13783: f64, t1926: f64, t221: f64, t25432: f64, t25806: f64, t6680: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88753 = 0.54831135561607547884e-2_f64 * t23384 * t25827;
    let t88758 = 0.18277045187202515961e-2_f64 * t23384 * t25436;
    let t88772 = t23328 * t23394;
    let t88804 = t1054 * t4693;
    let t88810 = t1926 * t221 * t13783;
    let t88812 = 0.24369393582936687948e-2_f64 * t88810 * t25432;
    let t88845 = 0.14621636149762012769e-1_f64 * t6680 * t25806;
    (t88753, t88758, t88772, t88804, t88810, t88812, t88845)
}
