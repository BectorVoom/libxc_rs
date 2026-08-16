//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3299/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3299<F: Float>(t2782: F, t4086: F, t543: F, t86506: F, t86445: F, t1399: F, t14255: F, t21981: F, t21990: F, t47417: F, t47442: F, t49276: F, t49361: F, t5745: F, t5755: F, t6862: F, t6874: F, t75252: F, t820: F, t86441: F, t86597: F) -> F {
    let t86604 = t2782 * t4086 * t86506 * t543;
    let t86608 = t2782 * t4086 * t86445 * t543;
    let t86616 = F::cast_from(0.58911598146606471821e-3_f64) * t49361 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t49276 * t6862 - t47417 - F::cast_from(0.29272321618148349057e-1_f64) * t75252 - F::cast_from(0.54878743191129263322e-2_f64) * t86597 + F::cast_from(0.79025390195226139182e1_f64) * t5745 * t21981 * t21990 + F::cast_from(0.16463622957338778997e-1_f64) * t86604 + F::cast_from(0.54878743191129263322e-2_f64) * t86608 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t14255 * t6874 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t86441 * t1399 + t47442;
    t86616
}
