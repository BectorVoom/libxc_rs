//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2144/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2144(t10868: f64, t820: f64, t3070: f64, t3072: f64, t10489: f64, t3117: f64, t1015: f64, t10472: f64, t42559: f64, t10870: f64, t3048: f64, t204: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43198 = t820 * t10868;
    let t43200 = t3070 * t43198 * t3072;
    let t43206 = t3117 * t10489;
    let t43211 = t10472 * t1015 * t42559;
    let t43214 = t3048 * t10870;
    let t43216 = t204 * t376;
    (t43198, t43200, t43206, t43211, t43214, t43216)
}
