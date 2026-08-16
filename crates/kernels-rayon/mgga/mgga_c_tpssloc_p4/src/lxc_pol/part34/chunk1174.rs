//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1174/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1174(t20800: f64, t6552: f64, t6637: f64, t6638: f64, t1888: f64, t20873: f64, t6646: f64, t1510: f64, t25038: f64, t98336: f64, t22986: f64, t22996: f64, t25249: f64, t5585: f64) -> (f64, f64, f64, f64) {
    let t105531 = t6552 * t6637 * t6638 * t20800;
    let t105543 = t1888 * t6646 * t20873;
    let t105547 = t25038 * t6646 * t98336 * t1510;
    let t105551 = t22986 * t22996 * t25249 * t5585;
    (t105531, t105543, t105547, t105551)
}
