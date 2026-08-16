//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 983/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk983(t114770: f64, t22986: f64, t25054: f64, t25038: f64, t25040: f64, t2717: f64, t7841: f64, t1888: f64, t23270: f64, t865: f64, t31337: f64, t4255: f64) -> (f64, f64, f64, f64) {
    let t121336 = t22986 * t114770 * t25054;
    let t121339 = t25038 * t114770 * t25040;
    let t121349 = t2717 * t7841;
    let t121352 = t1888 * t23270 * t121349 * t865;
    let t121362 = t25038 * t23270 * t31337 * t4255;
    (t121336, t121339, t121352, t121362)
}
