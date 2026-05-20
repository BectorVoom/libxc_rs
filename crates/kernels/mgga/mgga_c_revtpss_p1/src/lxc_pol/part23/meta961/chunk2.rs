//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3247/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3247<F: Float>(t85420: F, t85440: F, t1424: F, t14299: F, t22415: F, t22971: F, t23043: F, t4071: F, t4076: F, t46359: F, t47764: F, t47772: F, t47781: F, t47785: F, t47786: F, t5715: F, t5774: F, t6896: F, t6918: F, t73587: F, t73590: F, t73593: F, t73598: F) -> (F, F) {
    let t85442 = t85420 / F::new(2.0) + t85440 / F::new(2.0);
    let t85466 = F::cast_from(0.58911598146606471821e-3_f64) * t47764 - F::cast_from(0.39029762157531132076e-1_f64) * t73587 + F::cast_from(0.16463622957338778996e-1_f64) * t73590 + F::cast_from(0.39029762157531132074e-2_f64) * t73593 + F::cast_from(0.29272321618148349057e-1_f64) * t73598 - F::cast_from(0.65854491829355115987e0_f64) * t4071 * t23043 + F::cast_from(0.39512695097613069591e1_f64) * t4071 * t22971 + F::cast_from(0.33133632253434461091e-3_f64) * t47772 + F::cast_from(0.39512695097613069592e1_f64) * t5715 * t22415 + F::cast_from(0.39512695097613069591e1_f64) * t14299 * t6896 - F::cast_from(0.58911598146606471821e-3_f64) * t47781 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t4076 * t5774 * t6918 + t47785 - F::cast_from(0.78059524315062264151e-2_f64) * t47786 + t46359;
    (t85442, t85466)
}
