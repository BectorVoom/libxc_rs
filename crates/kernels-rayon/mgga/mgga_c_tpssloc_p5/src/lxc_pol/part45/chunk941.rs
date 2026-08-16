//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 941/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk941(t6665: f64, t868: f64, t25373: f64, t23285: f64, t28: f64, t1081: f64, t25927: f64, t113069: f64, t23788: f64, t2240: f64, t2244: f64, t32: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t113123 = t6665 * t868;
    let t113124 = t25373 * t113123;
    let t113741 = t28 * t23285;
    let t113751 = t1081 * t6665;
    let t113764 = t25927 * t113123;
    let t113772 = t23788 * t113069;
    let t113824 = t2240 * t32 * t2244;
    (t113123, t113124, t113741, t113751, t113764, t113772, t113824)
}
