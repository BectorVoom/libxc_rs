//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1502/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1502<F: Float>(t2212: F, t6936: F, t118089: F, t118091: F, t118094: F, t118099: F, t118106: F, t118629: F, t118957: F, t118962: F, t1456: F, t1458: F, t1464: F, t1914: F, t2205: F, t22571: F, t3: F, t31512: F, t31701: F, t31737: F, t575: F, t5808: F, t8417: F) -> F {
    let t118968 = t6936 * t2212;
    let t118975 = t1458 * (t118629 + t118962) + t118089 + t118091 + t118094 + t3 * t118957 * t575 + t2205 * t22571 + t118968 + t1456 * t31737 + t31701 * t1464 + t118099 + F::new(2.0) * t1914 * t31512 + F::new(2.0) * t8417 * t5808 + t118106;
    t118975
}
