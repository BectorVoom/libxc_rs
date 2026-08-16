//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 987/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk987<F: Float>(t2047: F, t28150: F, t28089: F, t7349: F, t7702: F, t7348: F, t7719: F, t1923: F, t2048: F, t25162: F, t26170: F, t26175: F, t26182: F, t26190: F, t26207: F, t28093: F, t28133: F, t28147: F, t28154: F, t6954: F, t6963: F, t7343: F, t7352: F, t7964: F) -> F {
    let t28628 = t2047 * t28150;
    let t28635 = t2047 * t28089;
    let t28638 = t7702 * t7349;
    let t28640 = t7348 * t7719;
    let t28641 = t1923 * t28640;
    let t28649 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t28133 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t26170 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t26190 + t26207 + F::cast_from(10.0_f64) * t26175 * t28147 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t28628 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t26182 + t6954 * t7964 / F::cast_from(3.0_f64) + t1923 * t28635 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t28638 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t28641 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6963 * t7964 + t28093 * t2048 / F::cast_from(3.0_f64) + t7702 * t7352 / F::cast_from(3.0_f64);
    t28649
}
