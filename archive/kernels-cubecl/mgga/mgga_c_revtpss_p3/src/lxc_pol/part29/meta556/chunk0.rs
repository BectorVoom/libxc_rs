//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1898/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1898<F: Float>(t25877: F, t97699: F, t14224: F, t689: F, t1398: F, t543: F, t5774: F, t1903: F, t4056: F, t25304: F, t27883: F, t25898: F) -> (F, F, F, F, F, F) {
    let t97700 = t97699 * t25877;
    let t97705 = t14224 * t689;
    let t97737 = t5774 * t1398 * t543;
    let t97742 = t1903 * t4056 * t543;
    let t97799 = t25304 * t27883;
    let t97802 = t97699 * t25898;
    (t97700, t97705, t97737, t97742, t97799, t97802)
}
