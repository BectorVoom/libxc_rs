//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1329/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1329<F: Float>(t10415: F, t1310: F, t13207: F, t13216: F, t2127: F, t2163: F, t2320: F, t26800: F, t3813: F, t508: F, t7584: F, t7586: F, t7683: F, t95017: F, t95020: F, t95023: F, t95025: F, t95032: F, t95036: F, t95038: F, t95040: F, t95042: F, t95046: F, t95049: F, t95056: F, t95058: F, t96834: F) -> F {
    let t97550 = -t10415 * t2163 - F::new(3.0) * t1310 * t26800 - t13207 * t2127 - F::new(6.0) * t13216 * t7586 - F::new(3.0) * t2320 * t7683 - F::new(3.0) * t3813 * t7584 - t508 * t96834 - t95017 + t95020 + t95023 + t95025 - t95032 + t95036 - t95038 - t95040 - t95042 + t95046 - t95049 + t95056 + t95058;
    t97550
}
