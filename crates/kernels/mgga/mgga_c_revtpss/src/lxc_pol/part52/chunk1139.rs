//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1139/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1139<F: Float>(t121336: F, t121305: F, t32186: F, t7063: F, t119900: F, t121165: F, t240: F, t545: F, t1412: F, t844: F, t32291: F, t8591: F) -> (F, F, F, F, F) {
    let t121337 = F::new(0.1054086758983270768e-1) * t121336;
    let t121342 = t7063 * t121305 * t32186;
    let t121346 = t119900 * t545 * t240 * t121165;
    let t121354 = t844 * t1412;
    let t121356 = t8591 * t121354 * t32291;
    (t121337, t121342, t121346, t121354, t121356)
}
