//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1002/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1002(t2639: f64, t3431: f64, t7284: f64, t787: f64, t13008: f64, t2087: f64, t4614: f64, t13133: f64, t2197: f64, t43901: f64, t43904: f64, t43908: f64, t43909: f64, t43910: f64, t43911: f64, t43913: f64, t43915: f64, t43918: f64, t43919: f64, t43922: f64, t43924: f64, t43926: f64, t43928: f64, t43931: f64, t43935: f64, t43938: f64) -> f64 {
    let t43941 = t787 * t7284 * t3431 * t2639;
    let t43944 = t2087 * t4614 * t13008;
    let t43946 = t2197 * t13133;
    let t43948 = 0.47667319935800568892e0_f64 * t43901 - 0.51123901271894332901e0_f64 * t43904 + t43908 - t43909 + t43910 + t43911 - t43913 + t43915 + t43918 - 0.38342925953920749676e0_f64 * t43919 - 0.38342925953920749676e0_f64 * t43922 + t43924 - t43926 - t43928 - t43931 - t43935 - t43938 - 0.50050685932590597338e1_f64 * t43941 - 0.18404604457881959845e2_f64 * t43944 + 0.23005755572352449806e2_f64 * t43946;
    t43948
}
