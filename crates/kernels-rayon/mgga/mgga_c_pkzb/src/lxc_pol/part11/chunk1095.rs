//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1095/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1095(t19758: f64, t1625: f64, t2557: f64, t83: f64, t1008: f64, t5075: f64, t1548: f64, t2607: f64, t1009: f64, t4882: f64, t5137: f64, t1508: f64, t7035: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19759 = 0.35089341735807877242e1_f64 * t19758;
    let t19775 = t83 * t2557 * t1625;
    let t19776 = 3.0_f64 * t19775;
    let t19778 = t83 * t1008 * t5075;
    let t19797 = t1548 * t2607;
    let t19798 = 96.0_f64 * t19797;
    let t19803 = t4882 * t1009;
    let t19804 = 240.0_f64 * t19803;
    let t19805 = t5137 * t1009;
    let t19822 = t7035 * t1508;
    (t19759, t19776, t19778, t19798, t19804, t19805, t19822)
}
