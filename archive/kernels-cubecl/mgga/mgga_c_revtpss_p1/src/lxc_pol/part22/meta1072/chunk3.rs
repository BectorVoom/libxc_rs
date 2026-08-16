//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3845/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3845<F: Float>(t22020: F, t2661: F, t5675: F, t9934: F, t22267: F, t9976: F, t13847: F, t1399: F, t73731: F, t9816: F, t22046: F, t22074: F, t3934: F, t3936: F, t4057: F, t48143: F, t48445: F, t48449: F, t48453: F, t48458: F, t48462: F, t9955: F, t9956: F) -> F {
    let t73951 = t2661 * t9934 * t22020 * t5675;
    let t73953 = t9976 * t22267;
    let t73963 = t9816 * t13847 * t73731 * t1399;
    let t73973 = -F::cast_from(0.28582678745379824648e-4_f64) * t73951 + F::cast_from(0.13552000749142754193e-3_f64) * t73953 + F::cast_from(0.50820002809285328225e-3_f64) * t48143 - F::cast_from(0.25410001404642664112e-4_f64) * t48445 - F::cast_from(0.57165357490759649296e-4_f64) * t48449 + F::cast_from(0.14291339372689912324e-4_f64) * t48453 + F::cast_from(0.28582678745379824648e-3_f64) * t48458 - F::cast_from(0.57165357490759649296e-4_f64) * t48462 - F::cast_from(0.25410001404642664112e-4_f64) * t73963 - F::cast_from(0.42874018118069736972e-2_f64) * t3934 * t9955 * t22046 * t9956 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t22074 * t4057;
    t73973
}
