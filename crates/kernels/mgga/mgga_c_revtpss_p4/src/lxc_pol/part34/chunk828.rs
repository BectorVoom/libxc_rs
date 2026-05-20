//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 828/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk828<F: Float>(t11735: F, t345: F, t1014: F, t2852: F, t245: F, t3089: F, t3088: F, t3114: F, t271: F, t2857: F, t11144: F, t11150: F, t3252: F) -> (F, F, F, F, F, F) {
    let t11737 = F::new(5.0) / F::new(1296.0) * t345 * t11735;
    let t11765 = t1014 * t2852;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    let t11774 = t3114 * t11773;
    let t11821 = F::new(1.0) / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11827 = t3252 * t11150;
    (t11737, t11765, t11772, t11774, t11822, t11827)
}
