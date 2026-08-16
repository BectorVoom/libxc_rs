//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1351/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1351(t25082: f64, t49640: f64, t8717: f64, t25191: f64, t7235: f64, t2322: f64, t25861: f64, t13435: f64, t7003: f64, t25856: f64, t25188: f64, t7313: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95032 = 9.0_f64 * t25082 * t8717 * t49640;
    let t95036 = 18.0_f64 * t7235 * t25191;
    let t95038 = 12.0_f64 * t2322 * t25861;
    let t95040 = 12.0_f64 * t13435 * t7003;
    let t95042 = 6.0_f64 * t2322 * t25856;
    let t95046 = 3.0_f64 * t25188 * t7313;
    (t95032, t95036, t95038, t95040, t95042, t95046)
}
