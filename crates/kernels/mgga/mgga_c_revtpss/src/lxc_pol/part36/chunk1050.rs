//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1050/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1050<F: Float>(t24129: F, t24176: F, t1079: F, t1076: F, t11201: F, t16284: F, t1652: F, t1680: F, t1696: F, t20175: F, t20191: F, t23959: F, t24044: F, t24048: F, t24061: F, t24068: F, t3058: F, t342: F, t386: F, t4747: F, t4752: F, t4935: F, t6235: F, t6245: F, t6251: F, t6259: F, t6351: F, t6393: F) -> F {
    let t24177 = t24129 + t24176;
    let t24178 = t1079 * t24177;
    let t24185 = F::new(0.65854491829355115987e0) * t342 * t24044 - F::new(0.39512695097613069591e1) * t1076 * t24048 - F::new(0.19756347548806534796e1) * t4752 * t6393 + F::new(0.39512695097613069591e1) * t4747 * t6251 + F::new(0.39512695097613069591e1) * t4752 * t6351 + F::new(0.65854491829355115987e0) * t23959 * t386 + F::new(0.19756347548806534796e1) * t6235 * t1680 + F::new(0.39512695097613069591e1) * t3058 * t24061 - F::new(0.19756347548806534796e1) * t4935 * t6393 + F::new(0.39512695097613069591e1) * t16284 * t6245 - F::new(0.39512695097613069591e1) * t11201 * t24068 - F::new(0.19756347548806534796e1) * t4747 * t6259 - F::new(0.65854491829355115987e0) * t1076 * t24178 - F::new(0.39512695097613069591e1) * t20191 * t1652 - F::new(0.39512695097613069591e1) * t20175 * t1696;
    t24185
}
