//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2816/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2816<F: Float>(t231: F, t2782: F, t2783: F, t76169: F, t62615: F, t62619: F, t62626: F, t62630: F, t62633: F, t62635: F, t62639: F, t76153: F, t76158: F, t76163: F) -> F {
    let t76172 = t2782 * t2783 * t76169 * t231;
    let t76174 = F::cast_from(0.16463622957338778996e-1_f64) * t62615 + F::cast_from(0.29272321618148349057e-1_f64) * t62619 - F::cast_from(0.58544643236296698112e-1_f64) * t76153 + F::cast_from(0.32927245914677557992e-1_f64) * t62626 + F::cast_from(0.58544643236296698112e-1_f64) * t76158 + F::cast_from(0.16463622957338778997e-1_f64) * t76163 - F::cast_from(0.65854491829355115984e-1_f64) * t62630 + F::cast_from(0.39029762157531132076e-1_f64) * t62633 - F::cast_from(0.29272321618148349057e-1_f64) * t62635 + F::cast_from(0.16463622957338778996e-1_f64) * t62639 + F::cast_from(0.54878743191129263322e-2_f64) * t76172;
    t76174
}
