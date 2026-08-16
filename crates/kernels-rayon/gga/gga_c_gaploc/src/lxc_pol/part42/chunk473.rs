//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 473/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk473(t2021: f64, t6110: f64, t1858: f64, t935: f64, t1890: f64, t7291: f64, t5241: f64, t739: f64, t7068: f64, t2530: f64, t325: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7630 = t2021 * t6110;
    let t7634 = t1858 * t935;
    let t7659 = t1890 * t7291;
    let t7663 = t5241 * t7291;
    let t7667 = t739 * t7291;
    let t7671 = t739 * t7068;
    let t7675 = t1890 * t7068;
    let t7696 = t1890 * t2530;
    let t7784 = t883 * t325;
    (t7630, t7634, t7659, t7663, t7667, t7671, t7675, t7696, t7784)
}
