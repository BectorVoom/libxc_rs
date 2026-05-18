//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1039/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1039<F: Float>(t24253: F, t300: F, t1733: F, t20629: F, t5063: F, t6471: F, t16840: F, t6474: F, t24220: F, t3435: F, t12248: F, t5071: F, t6449: F) -> (F, F, F, F, F, F) {
    let t24255 = F::new(0.19751673498613801407e-1) * t300 * t24253;
    let t24257 = F::new(3.0) * t20629 * t1733;
    let t24259 = F::new(3.0) * t5063 * t6471;
    let t24261 = F::new(0.48245938496077605201e2) * t16840 * t6474;
    let t24262 = t24220 * t3435;
    let t24264 = F::new(0.96491876992155210402e2) * t12248 * t24262;
    let t24265 = t5071 * t6449;
    (t24255, t24257, t24259, t24261, t24264, t24265)
}
