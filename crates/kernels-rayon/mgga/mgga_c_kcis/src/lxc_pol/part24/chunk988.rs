//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 988/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk988(t19825: f64, t19827: f64, t19829: f64, t19831: f64, t19833: f64, t19835: f64, t19838: f64, t19841: f64, t19842: f64, t19845: f64, t20211: f64, t187: f64, t20813: f64) -> f64 {
    let t20814 = -t19825 + t19827 + t19829 - t19831 + t19833 - t19835 + t19838 - t19841 + t19842 - t19845 + t20211;
    let t20817 = t19825 - t19827 - t19829 + t19831 - t19833 + t19835 - t19838 + t19841 - t19842 + t19845 - t20211 + t187 * (t20813 + t20814);
    t20817
}
