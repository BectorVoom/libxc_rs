//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1691/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1691(t3792: f64, t6414: f64, t2632: f64, t5611: f64, t107: f64, t240: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64, t2240: f64, t608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20473 = t3792 * t6414;
    let t20986 = t2632 * t5611;
    let t22468 = t240 * t107;
    let t22469 = 11.0_f64 / 9.0_f64 * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22473 = t63 * t2331;
    let t22510 = 88.0_f64 / 9.0_f64 * t240;
    let t22549 = t2240 * t608;
    (t20473, t20986, t22469, t22470, t22471, t22473, t22510, t22549)
}
