//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 657/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk657<F: Float>(t10862: F, t10864: F, t10866: F, t10869: F, t10871: F, t10873: F, t9812: F, t9814: F, t9822: F, t9826: F, t9831: F, t9835: F, t9837: F, t9845: F, t9848: F) -> F {
    let t12199 = -t10862 + t10864 + t10866 - t10869 + t10871 + t10873 + t9812 + F::cast_from(0.51123901271894332903e0_f64) * t9814 - t9822 + t9826 + F::cast_from(0.38342925953920749677e0_f64) * t9831 - F::cast_from(0.85206502119823888171e-1_f64) * t9835 + F::cast_from(0.38342925953920749677e0_f64) * t9837 - F::cast_from(0.38342925953920749677e0_f64) * t9845 - F::cast_from(0.38342925953920749677e0_f64) * t9848;
    t12199
}
