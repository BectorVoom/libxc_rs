//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 686/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk686(t12782: f64, t471: f64, t3334: f64, t871: f64, t3113: f64, t984: f64, t12383: f64, t12386: f64, t12397: f64, t12400: f64, t12412: f64) -> f64 {
    let t12783 = t12782 * t471;
    let t12784 = t3334 * t871;
    let t12785 = t984 * t3113;
    let t12787 = 9.0_f64 / 256.0_f64 * t12383;
    let t12788 = 9.0_f64 / 8192.0_f64 * t12386;
    let t12789 = 3.0_f64 / 8192.0_f64 * t12397;
    let t12790 = 3.0_f64 / 256.0_f64 * t12400;
    let t12791 = 2.0_f64 * t12412;
    let t12792 = t12783 + t12784 - t12785 / 2.0_f64 - t12787 - t12788 + t12789 + t12790 + t12791;
    t12792
}
