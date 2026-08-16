//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1462/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1462(t1527: f64, t2719: f64, t10110: f64, t225: f64, t4143: f64, t2742: f64, t2718: f64, t4265: f64, t798: f64, t4145: f64, t4142: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13049 = t1527 * t2719;
    let t13050 = t10110 * t13049;
    let t13053 = t4143 * t225;
    let t13058 = t1527 * t2742;
    let t13059 = t2718 * t13058;
    let t13062 = t798 * t4265;
    let t13065 = t4145 * t225;
    let t13068 = t4142 * t852;
    (t13049, t13050, t13053, t13058, t13059, t13062, t13065, t13068)
}
