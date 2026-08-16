//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1125/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1125(t441: f64, t4533: f64, t2169: f64, t1295: f64, t1657: f64, t8121: f64, t915: f64, t233: f64, t2209: f64, t4534: f64, t235: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27746 = t4533 * t441;
    let t27747 = t2169 * t27746;
    let t27749 = t1657 * t1295;
    let t27750 = t2169 * t27749;
    let t27752 = t915 * t8121;
    let t27753 = t233 * t27752;
    let t27755 = t4534 * t2209;
    let t27756 = t233 * t27755;
    let t27758 = t235 * t5398;
    (t27746, t27747, t27749, t27750, t27752, t27753, t27755, t27756, t27758)
}
