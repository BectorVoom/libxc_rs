//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2125/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2125(t1843: f64, t5920: f64, t1513: f64, t5891: f64, t10208: f64, t4263: f64, t5915: f64, t1504: f64, t5895: f64, t10227: f64, t4269: f64, t5823: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22578 = t1843 * t5920;
    let t22589 = t5891 * t1513;
    let t22590 = t10208 * t22589;
    let t22593 = t4263 * t5915;
    let t22596 = t5895 * t1504;
    let t22597 = t10227 * t22596;
    let t22600 = t4269 * t5823;
    (t22578, t22589, t22590, t22593, t22596, t22597, t22600)
}
