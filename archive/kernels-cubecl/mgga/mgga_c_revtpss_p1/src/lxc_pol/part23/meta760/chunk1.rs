//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2555/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2555<F: Float>(t1062: F, t43154: F, t16088: F, t342: F, t380: F, t16219: F, t3241: F, t1063: F, t11262: F, t4802: F, t4807: F, t11773: F, t15925: F) -> (F, F, F, F, F, F) {
    let t54982 = t43154 * t1062;
    let t55011 = t342 * t380 * t16088;
    let t55033 = t3241 * t16219;
    let t55034 = t55033 / F::cast_from(162.0_f64);
    let t55061 = t1063 * t11262 * t4802;
    let t55062 = F::cast_from(0.19055119163586549765e-3_f64) * t55061;
    let t55064 = t1063 * t11262 * t4807;
    let t55065 = F::cast_from(0.15879265969655458138e-3_f64) * t55064;
    let t55141 = t15925 * t11773;
    (t54982, t55011, t55034, t55062, t55065, t55141)
}
