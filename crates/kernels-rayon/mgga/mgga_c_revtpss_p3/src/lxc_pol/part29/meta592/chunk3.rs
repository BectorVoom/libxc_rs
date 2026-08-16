//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1970/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1970(t102165: f64, t25904: f64, t1445: f64, t28824: f64, t689: f64, t102274: f64, t25878: f64, t102100: f64, t26069: f64, t26231: f64, t98380: f64, t13730: f64, t2098: f64, t2782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102346 = 0.14456046980341999104e-1_f64 * t25904 * t102165;
    let t102361 = 0.10975748638225852664e-1_f64 * t689 * t28824 * t1445;
    let t102363 = 0.51405703062096148812e-1_f64 * t25878 * t102274;
    let t102364 = t26069 * t102100;
    let t102367 = 0.25702851531048074406e-1_f64 * t98380 * t26231;
    let t102372 = 0.21951497276451705328e-1_f64 * t2782 * t2098 * t13730;
    (t102346, t102361, t102363, t102364, t102367, t102372)
}
