//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3845/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3845(t22020: f64, t2661: f64, t5675: f64, t9934: f64, t22267: f64, t9976: f64, t13847: f64, t1399: f64, t73731: f64, t9816: f64, t22046: f64, t22074: f64, t3934: f64, t3936: f64, t4057: f64, t48143: f64, t48445: f64, t48449: f64, t48453: f64, t48458: f64, t48462: f64, t9955: f64, t9956: f64) -> f64 {
    let t73951 = t2661 * t9934 * t22020 * t5675;
    let t73953 = t9976 * t22267;
    let t73963 = t9816 * t13847 * t73731 * t1399;
    let t73973 = -0.28582678745379824648e-4_f64 * t73951 + 0.13552000749142754193e-3_f64 * t73953 + 0.50820002809285328225e-3_f64 * t48143 - 0.25410001404642664112e-4_f64 * t48445 - 0.57165357490759649296e-4_f64 * t48449 + 0.14291339372689912324e-4_f64 * t48453 + 0.28582678745379824648e-3_f64 * t48458 - 0.57165357490759649296e-4_f64 * t48462 - 0.25410001404642664112e-4_f64 * t73963 - 0.42874018118069736972e-2_f64 * t3934 * t9955 * t22046 * t9956 + 0.85748036236139473944e-3_f64 * t3934 * t3936 * t22074 * t4057;
    t73973
}
