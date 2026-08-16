//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1816/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1816(t28938: f64, t7900: f64, t2107: f64, t22475: f64, t1502: f64, t1519: f64, t1843: f64, t2014: f64, t2052: f64, t2089: f64, t28653: f64, t30558: f64, t30563: f64, t30571: f64, t30578: f64, t30581: f64, t30584: f64, t30586: f64, t30589: f64, t30612: f64, t4248: f64, t508: f64, t569: f64, t5877: f64, t5884: f64, t5921: f64, t651: f64, t6765: f64, t7359: f64, t7732: f64, t7969: f64, t7984: f64, t7988: f64, t8065: f64) -> (f64, f64, f64) {
    let t30614 = t28938 * t7900;
    let t30617 = t2107 * t22475;
    let t30625 = -2.0_f64 * t7969 * t1843 - 2.0_f64 * t651 * t30558 - 4.0_f64 * t7732 * t7984 - 2.0_f64 * t651 * t30563 - 2.0_f64 * t651 * t30571 - 4.0_f64 * t4248 * t7988 - 4.0_f64 * t28653 * t1519 - 4.0_f64 * t651 * t30578 + 3.0_f64 * t2014 * t30581 - t2014 * t30584 + 6.0_f64 * t2014 * t30586 - 2.0_f64 * t30589 * t508 - t5877 * t2089 - 2.0_f64 * t1502 * t8065 + t30612 * t569 + 6.0_f64 * t2014 * t30614 + 2.0_f64 * t2014 * t30617 - t2052 * t6765 - 2.0_f64 * t5884 * t2089 - 2.0_f64 * t7359 * t5921;
    (t30614, t30617, t30625)
}
