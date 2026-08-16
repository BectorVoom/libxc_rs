//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 942/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk942(t2685: f64, t3916: f64, t1464: f64, t948: f64, t345: f64, t836: f64, t2724: f64, t3962: f64, t8983: f64, t2740: f64, t3944: f64, t2459: f64, t969: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11562 = t2685 * t3916 / 162.0_f64;
    let t11568 = t1464 * t948;
    let t11569 = t345 * t836;
    let t11575 = t1464 * t2724;
    let t11584 = t8983 * t3962;
    let t11586 = t2740 * t11584 / 3456.0_f64;
    let t11588 = t8983 * t3944;
    let t11590 = t2740 * t11588 / 3456.0_f64;
    let t11621 = t969 * t2459;
    (t11562, t11568, t11569, t11575, t11586, t11590, t11621)
}
