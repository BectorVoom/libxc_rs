//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1962/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1962(t2031: f64, t90090: f64, t26012: f64, t7031: f64, t22549: f64, t90094: f64, t26009: f64, t84219: f64, t90247: f64, t23963: f64, t23970: f64, t26016: f64, t26954: f64, t83722: f64, t83778: f64, t84183: f64, t84190: f64, t90076: f64, t90080: f64, t90114: f64) -> f64 {
    let t92040 = t2031 * t90090;
    let t92047 = t7031 * t26012;
    let t92049 = 160.0_f64 / 9.0_f64 * t22549 * t92047;
    let t92052 = t2031 * t90094;
    let t92056 = 160.0_f64 / 3.0_f64 * t84219 * t26009;
    let t92057 = t2031 * t90247;
    let t92068 = 20.0_f64 / 3.0_f64 * t22549 * t92040 + 20.0_f64 * t23963 * t90076 + 10.0_f64 * t23963 * t90080 - t92049 + 10.0_f64 / 3.0_f64 * t83778 * t26954 + 20.0_f64 / 3.0_f64 * t22549 * t92052 - t92056 + 20.0_f64 / 3.0_f64 * t22549 * t92057 + 10.0_f64 / 3.0_f64 * t26016 * t84183 + 20.0_f64 / 3.0_f64 * t90114 * t23970 + 20.0_f64 * t84190 * t26009 + 20.0_f64 / 3.0_f64 * t83722 * t26954;
    t92068
}
