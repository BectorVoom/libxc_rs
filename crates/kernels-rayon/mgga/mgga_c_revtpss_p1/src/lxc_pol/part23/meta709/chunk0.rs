//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2464/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2464(t1904: f64, t2439: f64, t9640: f64, t5718: f64, t9292: f64, t14274: f64, t2435: f64, t10175: f64, t14090: f64, t14085: f64, t14104: f64, t47520: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47800 = t2439 * t9640 * t1904;
    let t47802 = t9292 * t5718;
    let t47805 = t2435 * t14274;
    let t47806 = 0.21951497276451705329e-1_f64 * t47805;
    let t47813 = t10175 * t14090;
    let t47814 = 0.39029762157531132076e-1_f64 * t47813;
    let t47834 = t2435 * t14085;
    let t47835 = 0.21951497276451705329e-1_f64 * t47834;
    let t47837 = t47520 * t14104;
    (t47800, t47802, t47806, t47814, t47835, t47837)
}
