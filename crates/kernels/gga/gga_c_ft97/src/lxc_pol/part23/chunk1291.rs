//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1291/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1291<F: Float>(t2469: F, t31060: F, t1882: F, t31289: F, t10007: F, t111363: F, t111443: F, t111452: F, t111466: F, t121882: F, t1424: F, t17790: F, t17794: F, t18391: F, t18459: F, t18725: F, t18760: F, t1901: F, t242: F, t24789: F, t2574: F, t28386: F, t31220: F, t446: F, t5166: F, t53797: F, t54032: F, t6088: F, t729: F, t97733: F, t98061: F) -> (F, F) {
    let t125030 = t2469 * t31060;
    let t125040 = t1882 * t31289;
    let t125050 = 2.0 / 9.0 * t1901 * t97733 * t5166 + t111443 + 2.0 / 9.0 * t1901 * t24789 * t18725 - 2.0 / 3.0 * t446 * t242 * t121882 - t111452 + t446 * t729 * t18391 * t6088 / 3.0 - 2.0 / 3.0 * t446 * t2574 * t2469 * t31220 + t111466 - 4.0 / 81.0 * t98061 - t446 * t242 * t125030 / 3.0 + 4.0 / 9.0 * t53797 * t111363 * t17790 - 4.0 / 27.0 * t54032 * t111363 * t17794 - 2.0 / 9.0 * t125040 - t446 * t729 * t18760 * t1424 / 3.0 + 2.0 / 9.0 * t1901 * t10007 * t28386 * t18459;
    (t125030, t125050)
}
