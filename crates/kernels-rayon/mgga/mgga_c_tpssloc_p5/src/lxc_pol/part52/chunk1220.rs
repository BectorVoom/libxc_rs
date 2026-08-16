//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1220/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1220(t1983: f64, t33129: f64, t191: f64, t192: f64, t7681: f64, t2020: f64, t3701: f64, t7752: f64, t2019: f64, t1873: f64, t24999: f64, t33085: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33131 = 3.0_f64 * t1983 * t33129;
    let t33133 = t7681 * t191 * t192;
    let t33134 = t33133 * t2020;
    let t33136 = t3701 * t7752;
    let t33137 = t2019 * t33136;
    let t33139 = 2.0_f64 * t1983 * t33137;
    let t33142 = t24999 * t1873;
    let t33144 = t33085 * t1873;
    (t33131, t33133, t33134, t33136, t33137, t33139, t33142, t33144)
}
