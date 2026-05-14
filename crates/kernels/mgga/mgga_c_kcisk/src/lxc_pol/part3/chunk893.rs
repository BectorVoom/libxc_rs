//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 893/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk893<F: Float>(t12902: F, t12904: F, t12907: F, t12914: F, t13018: F, t13026: F, t13039: F, t13050: F, t13053: F, t13056: F, t13060: F, t13123: F, t14817: F, t14821: F, t14824: F, t14828: F, t14842: F, t4436: F, t4461: F, t4471: F, t4478: F, t516: F) -> (F,) {
    let t14846 = -6.0 * t4436 * t14817 + 0.96494049533612093922e2 * t4461 * t14821 - 0.35089340384731224426e1 * t4471 * t14824 + 0.51947267698127589897e2 * t4478 * t14828 + t13123 + t13056 - t13060 - t12902 - 0.3109e-1 * t14842 * t516 - 0.19751789702565206229e-1 * t13039 - t12904 - t12907 + t12914 - t13018 - t13026 + t13050 - t13053;
    (t14846,)
}
