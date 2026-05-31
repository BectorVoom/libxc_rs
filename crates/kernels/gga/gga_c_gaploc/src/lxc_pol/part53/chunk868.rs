//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 868/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk868<F: Float>(t1382: F, t2902: F, t3207: F, t12862: F, t4342: F, t10301: F, t6556: F, t27232: F, t3145: F, t8045: F, t9256: F, t12856: F, t17293: F, t605: F) -> (F, F, F, F, F, F) {
    let t42494 = F::cast_from(2.0_f64) * t1382 * t2902 * t3207;
    let t42496 = F::cast_from(2.0_f64) * t4342 * t12862;
    let t42501 = F::cast_from(4.0_f64) * t6556 * t10301;
    let t42503 = F::cast_from(2.0_f64) * t27232 * t3145;
    let t42506 = F::cast_from(4.0_f64) * t8045 * t9256;
    let t42509 = F::cast_from(24.0_f64) * t17293 * t12856 * t605;
    (t42494, t42496, t42501, t42503, t42506, t42509)
}
