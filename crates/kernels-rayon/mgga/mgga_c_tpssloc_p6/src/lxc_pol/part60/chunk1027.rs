//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1027/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1027(t28025: f64, t7042: f64, t28827: f64, t8607: f64, t33336: f64, t7685: f64, t28821: f64, t8644: f64, t1799: f64, t22574: f64, t26558: f64, t33221: f64) -> (f64, f64, f64, f64, f64) {
    let t128543 = 2.0_f64 * t7042 * t28025;
    let t128549 = 6.0_f64 * t8607 * t28827;
    let t128551 = 2.0_f64 * t7685 * t33336;
    let t128552 = t28821 * t8644;
    let t128562 = 12.0_f64 * t22574 * t26558 * t33221 * t1799;
    (t128543, t128549, t128551, t128552, t128562)
}
