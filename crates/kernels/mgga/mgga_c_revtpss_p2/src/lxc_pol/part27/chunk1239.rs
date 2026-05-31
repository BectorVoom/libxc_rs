//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1239/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1239<F: Float>(t1113: F, t2394: F, t1940: F, t1963: F, t2403: F, t25206: F, t25436: F, t25440: F, t25760: F, t25763: F, t25784: F, t27158: F, t27382: F, t33: F, t3351: F, t4541: F, t7087: F, t7091: F, t92819: F, t93397: F, t9357: F, t94228: F, t94231: F, t94234: F, t94240: F, t94246: F, t94255: F, t94259: F) -> F {
    let t94262 = t1113 * t2394;
    let t94272 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t25436 * t1113 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t25206 * t94228 + F::cast_from(9.0_f64) * t25206 * t94231 + F::cast_from(3.0_f64) * t27382 * t94234 + t1940 * t1963 * t9357 / F::cast_from(2.0_f64) - F::cast_from(9.0_f64) * t27158 * t94240 - F::cast_from(9.0_f64) * t92819 * t25760 - F::cast_from(9.0_f64) * t25206 * t94246 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7087 * t3351 + t1940 * t93397 * t33 / F::cast_from(2.0_f64) - t1940 * t7091 * t94255 / F::cast_from(2.0_f64) - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t25206 * t94259 + F::cast_from(9.0_f64) * t4541 * t1963 * t94262 + F::cast_from(9.0_f64) * t2403 * t7087 * t25763 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t25440 * t25784;
    t94272
}
