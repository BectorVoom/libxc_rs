//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1140/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1140(t19754: f64, t1009: f64, t5142: f64, t1639: f64, t7035: f64, t2706: f64, t5165: f64, t639: f64, t7177: f64, t1625: f64, t2557: f64, t83: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19755 = 24.0_f64 * t19754;
    let t19756 = t5142 * t1009;
    let t19757 = 144.0_f64 * t19756;
    let t19758 = t7035 * t1639;
    let t19759 = 0.35089341735807877242e1_f64 * t19758;
    let t19766 = t2706 * t5165;
    let t19770 = t7177 * t639;
    let t19775 = t83 * t2557 * t1625;
    (t19755, t19757, t19759, t19766, t19770, t19775)
}
