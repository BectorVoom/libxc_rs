//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 895/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk895(t939: f64, t348: f64, t2717: f64, t328: f64, t356: f64, t353: f64, t8550: f64, t2724: f64, t345: f64, t2716: f64, t941: f64, t2668: f64, t917: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8551 = t939 * t939;
    let t8552 = 1.0_f64 / t8551;
    let t8553 = t8552 * t348;
    let t8556 = 1.0_f64 / t2717 / t356 / t328;
    let t8557 = t353 * t8556;
    let t8559 = t8550 * t8553 * t8557;
    let t8561 = t2724 * t345;
    let t8568 = t8550 * t2716 * t8557;
    let t8577 = t8550 * t941 * t8557;
    let t8588 = t917 * t2668;
    (t8552, t8556, t8559, t8561, t8568, t8577, t8588)
}
