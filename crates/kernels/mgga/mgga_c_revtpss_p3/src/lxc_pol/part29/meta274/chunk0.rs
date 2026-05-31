//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1129/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1129<F: Float>(t5: F, t114: F, t1923: F, t2048: F, t7343: F, t7351: F, t7702: F, t7706: F, t7709: F, t7964: F, t117: F, t1843: F, t2055: F, t7370: F, t7738: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t115 = F::cast_from(1.0_f64) < t114;
    let t7968 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t7702 * t2048 / F::cast_from(3.0_f64) - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t7706 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t2048 - t7351 + t1923 * t7964 / F::cast_from(3.0_f64));
    let t7969 = t7968 * t117;
    let t7978 = t1843 * t2055;
    let t7983 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t7370 - t7738 / F::cast_from(4.0_f64));
    (t7968, t7969, t7978, t7983)
}
