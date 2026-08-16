//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2265/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2265<F: Float>(t23164: F, t7479: F, t86893: F, t17063: F, t23278: F, t25168: F, t5637: F, t82294: F, t87748: F, t87902: F, t87911: F, t87927: F, t87932: F, t92954: F, t92961: F, t99033: F) -> F {
    let t99036 = t23164 * t86893 * t7479;
    let t99038 = -t92954 + t87902 + t87911 - t92961 - F::cast_from(0.49348022005446793095e-1_f64) * t87927 + F::cast_from(24.0_f64) * t25168 * t87748 * t17063 + F::cast_from(2.0_f64) * t23278 * t5637 - F::cast_from(0.52089578783527170488e-1_f64) * t82294 - F::cast_from(0.3289868133696452873e-1_f64) * t99033 + F::cast_from(0.16449340668482264365e-1_f64) * t99036 - t87932;
    t99038
}
