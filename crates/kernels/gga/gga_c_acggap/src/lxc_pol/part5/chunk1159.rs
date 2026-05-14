//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1159/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1159<F: Float>(t13092: F, t6328: F, t6361: F, t997: F, t14056: F, t5903: F, t3379: F, t6161: F, t17039: F, t6339: F, t1083: F, t1165: F, t1173: F, t16059: F, t175: F, t18578: F, t18580: F, t18582: F, t18584: F, t1889: F, t21689: F, t372: F, t398: F, t418: F, t5784: F, t955: F) -> (F,) {
    let t24001 = t13092 * t6328;
    let t24003 = t997 * t6361;
    let t24009 = t14056 * t5903;
    let t24011 = t3379 * t6161;
    let t24013 = t17039 * t6339;
    let t24024 = 0.85748036236139473944e-3 * t1173 * t1165 * t1889 * t955 - 0.64025200389650807212e-1 * t24001 + 0.24009450146119052704e0 * t24003 + 0.18007087609589289528e0 * t418 * t16059 * t175 * t21689 + 0.13719685797782315831e-1 * t24009 + 0.17149607247227894789e-2 * t24011 + 0.10289764348336736873e-1 * t24013 - 0.17149607247227894789e-2 * t418 * t398 * t1083 * t5784 * t372 - 0.64025200389650807212e-1 * t18578 - 0.32012600194825403606e-1 * t18580 + 0.64025200389650807212e-1 * t18582 + 0.32012600194825403606e-1 * t18584;
    (t24024,)
}
