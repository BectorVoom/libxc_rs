//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1204/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1204<F: Float>(t1882: F, t31222: F, t31273: F, t27889: F, t3977: F, t31268: F, t10007: F, t109848: F, t110588: F, t110626: F, t110665: F, t1168: F, t14159: F, t18123: F, t18433: F, t18446: F, t1901: F, t242: F, t2469: F, t2599: F, t27742: F, t28153: F, t31155: F, t31239: F, t3864: F, t446: F, t5053: F, t53797: F, t6074: F, t6194: F, t684: F, t6921: F, t729: F, t762: F, t97705: F) -> (F, F) {
    let t122509 = t1882 * t31222;
    let t122518 = t1882 * t31273;
    let t122534 = t3977 * t27889;
    let t122539 = t1882 * t31268;
    let t122550 = -t446 * t729 * t6194 * t5053 / 3.0 - t110588 + 2.0 / 9.0 * t122509 + 2.0 / 9.0 * t1901 * t14159 * t28153 + 4.0 * t1901 * t109848 * t6921 * t3864 - 2.0 / 27.0 * t122518 + 4.0 / 9.0 * t53797 * t97705 * t18446 + 4.0 / 3.0 * t53797 * t110665 * t18433 - 2.0 / 9.0 * t1901 * t10007 * t31239 * t684 + t1901 * t2599 * t6074 * t18123 / 9.0 - 2.0 / 3.0 * t446 * t242 * t122534 + 8.0 / 81.0 * t110626 - 2.0 / 9.0 * t122539 + 2.0 / 3.0 * t446 * t729 * t2469 * t31155 + 2.0 / 3.0 * t446 * t729 * t762 * t27742 * t1168;
    (t122534, t122550)
}
