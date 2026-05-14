//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1197/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1197<F: Float>(t13776: F, t3861: F, t3975: F, t9504: F, t3912: F, t50887: F, t14138: F, t2409: F, t35890: F, t3965: F, t12243: F, t14121: F, t13772: F, t3200: F, t335: F, t338: F, t3917: F, t4183: F, t51957: F, t54536: F, t54538: F, t54567: F, t57581: F, t57584: F, t57588: F, t57593: F, t57595: F, t57598: F, t6793: F) -> (F,) {
    let t57602 = t13776 * t3975 * t3861 * t9504;
    let t57604 = t3912 * t50887;
    let t57605 = t57604 * t14138;
    let t57608 = t3965 * t2409 * t35890;
    let t57614 = t14121 * t12243;
    let t57618 = -t54536 + t54538 - 7.0 / 288.0 * t57581 + t57584 / 768.0 + t6793 * t57588 / 48.0 + t57593 / 768.0 + t57595 / 24.0 - t57598 / 48.0 + t54567 - t57602 / 384.0 - t57605 / 48.0 - t57608 / 96.0 + t51957 - t335 * t338 * t3200 * t4183 / 48.0 + t57614 / 16.0 - t3917 * t13772 / 96.0;
    (t57618,)
}
