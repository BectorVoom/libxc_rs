//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1337/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1337<F: Float>(t30565: F, t5: F, t104095: F, t1080: F, t118591: F, t118630: F, t118673: F, t119185: F, t119208: F, t119245: F, t119282: F, t119313: F, t119347: F, t119384: F, t119405: F, t119440: F, t119475: F, t119509: F, t119541: F, t121649: F, t16594: F, t16601: F, t17532: F, t17535: F, t17676: F, t184: F, t21: F, t24157: F, t27436: F, t27440: F, t3674: F, t3678: F, t4895: F, t4898: F, t5985: F, t650: F, t920: F) -> (F,) {
    let t121658 = t5 * t30565;
    let t121676 = t5 * t27436 * t920 / 2.0 + t5985 * t17676 / 4.0 + t5985 * t17535 / 4.0 + t5 * (t118591 + t118630 + t118673 + t119185 + t119208 + t119245 + t119282 + t119313 + t119347 + t119384 + t119405 + t119440 + t119475 + t119509 + t119541 + t121649) * t184 * t21 / 4.0 + t121658 * t650 / 4.0 + t104095 * t1080 / 2.0 + t5985 * t16601 / 4.0 + t24157 * t4895 / 4.0 + t27440 * t3674 / 2.0 + t27440 * t3678 + t24157 * t4898 / 2.0 + t5985 * t16594 / 4.0 + t5985 * t17532 / 2.0;
    (t121676,)
}
