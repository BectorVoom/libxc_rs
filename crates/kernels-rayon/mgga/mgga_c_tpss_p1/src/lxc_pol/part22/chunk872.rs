//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 872/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk872(t6419: f64, t1656: f64, t1838: f64, t5740: f64, t1639: f64, t520: f64, t5745: f64, t1773: f64, t522: f64, t1657: f64, t1772: f64, t1842: f64, t538: f64, t5739: f64, t5921: f64, t6260: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6420 = param_beta * t6419;
    let t6424 = t1838 * t1656;
    let t6425 = t5740 * t6424;
    let t6430 = t5745 * t1838 * t1639 * t520;
    let t6433 = t1773 * t522 * t6419;
    let t6435 = -t1657 * t5921 - t1772 * t6433 - t1842 * t6260 + t538 * t6420 + 2.0_f64 * t5739 * t6425 + t5739 * t6430;
    (t6420, t6424, t6425, t6430, t6433, t6435)
}
