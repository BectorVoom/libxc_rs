//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 800/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk800(t128: f64, t12929: f64, t10: f64, t8144: f64, t3637: f64, t978: f64, t102: f64, t974: f64, t8197: f64, t120: f64, t506: f64, t12898: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12930 = t128 * t12929;
    let t12931 = t10 * t12930;
    let t12934 = 0.97434166666666666666e0_f64 * t8144;
    let t12937 = t978 * t3637;
    let t12946 = 0.1753815e2_f64 * t102 * t974 * t3637;
    let t12947 = 0.19486833333333333333e1_f64 * t8197;
    let t12949 = t120 * t12929;
    let t12951 = 0.2923025e1_f64 * t102 * t12949;
    let t12952 = t506 * t12929;
    let t12955 = t5825 * t12898;
    (t12930, t12931, t12934, t12937, t12946, t12947, t12949, t12951, t12952, t12955)
}
