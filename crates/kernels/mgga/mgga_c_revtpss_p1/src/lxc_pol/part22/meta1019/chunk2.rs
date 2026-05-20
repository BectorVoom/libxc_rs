//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3533/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3533<F: Float>(t11933: F, t19976: F, t3115: F, t42793: F, t6272: F, t11922: F, t16081: F, t19749: F, t11866: F, t15758: F, t15917: F, t15922: F, t15926: F, t16052: F, t16078: F, t19726: F, t19758: F, t20066: F, t20070: F, t20075: F, t20105: F, t42643: F, t42830: F, t4891: F, t4896: F, t4907: F, t53855: F, t55958: F) -> F {
    let t67006 = t11933 * t19976;
    let t67015 = t3115 * t42793 * t6272;
    let t67025 = t16081 * t11922 * t19749;
    let t67031 = -F::cast_from(0.30488190661738479624e-2_f64) * t16052 * t19726 - F::cast_from(0.85748036236139473944e-3_f64) * t42643 * t20075 + F::cast_from(0.30488190661738479624e-2_f64) * t67006 - F::cast_from(0.85748036236139473944e-3_f64) * t53855 * t4907 - F::cast_from(0.85748036236139473944e-3_f64) * t15926 * t15922 - F::cast_from(0.42874018118069736972e-3_f64) * t15926 * t16078 + F::cast_from(0.95275595817932748827e-4_f64) * t67015 + F::cast_from(0.85748036236139473944e-3_f64) * t15758 * t20066 - F::cast_from(0.42874018118069736972e-3_f64) * t15917 * t20070 + F::cast_from(0.17149607247227894789e-2_f64) * t55958 * t4891 * t4896 + F::cast_from(0.17149607247227894789e-2_f64) * t67025 + F::cast_from(0.42874018118069736972e-3_f64) * t42830 * t19758 - F::cast_from(0.42874018118069736972e-3_f64) * t11866 * t20105;
    t67031
}
