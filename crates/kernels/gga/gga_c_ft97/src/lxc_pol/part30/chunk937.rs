//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 937/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk937<F: Float>(t1403: F, t35275: F, t681: F, t35262: F, t27929: F, t7437: F, t109755: F, t1449: F, t35617: F, t8392: F, t2567: F, t7484: F, t35621: F, t10007: F, t10085: F, t1091: F, t110401: F, t13839: F, t13885: F, t14127: F, t14163: F, t141713: F, t141722: F, t141744: F, t141746: F, t141752: F, t141902: F, t141989: F, t150120: F, t1901: F, t24737: F, t2599: F, t27983: F, t28108: F, t28140: F, t28299: F, t28300: F, t28364: F, t33476: F, t33715: F, t33760: F, t35553: F, t35562: F, t3837: F, t3842: F, t3859: F, t3864: F, t3880: F, t684: F) -> (F, F, F, F, F) {
    let t151421 = t1403 * t681 * t35275;
    let t151426 = t1403 * t681 * t35262;
    let t151428 = t7437 * t27929;
    let t151430 = t109755 * t1449;
    let t151461 = t8392 * t35617;
    let t151471 = t2567 * t7484;
    let t151483 = t8392 * t35621;
    let t151493 = 2.0 / 27.0 * t141722 - 2.0 * t1901 * t28140 * t33715 * t3837 - 2.0 / 3.0 * t1901 * t13885 * t141989 * t3842 - 2.0 / 9.0 * t1901 * t13839 * t33760 + t1901 * t10085 * t35562 / 9.0 + t1901 * t2599 * t141902 * t1091 / 9.0 + 4.0 / 9.0 * t141744 + 4.0 / 9.0 * t141746 - 2.0 / 9.0 * t1901 * t10007 * t35553 * t684 - 4.0 / 3.0 * t1901 * t13885 * t24737 * t28108 - 2.0 / 27.0 * t151461 - 4.0 * t1901 * t28299 * t28300 * t27983 + 2.0 * t1901 * t28140 * t141713 * t3859 - 2.0 / 3.0 * t1901 * t14127 * t151471 * t3864 - 2.0 / 3.0 * t1901 * t13885 * t141989 * t3859 - 4.0 / 3.0 * t1901 * t110401 * t28364 - t151483 / 27.0 - 2.0 / 27.0 * t141752 - 2.0 / 9.0 * t1901 * t14163 * t150120 - t1901 * t10007 * t33476 * t3880 / 9.0;
    (t151421, t151426, t151428, t151430, t151493)
}
