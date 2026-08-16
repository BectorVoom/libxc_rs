//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1070/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1070(t35621: f64, t8392: f64, t10007: f64, t10085: f64, t1091: f64, t110401: f64, t13839: f64, t13885: f64, t14127: f64, t14163: f64, t141713: f64, t141722: f64, t141744: f64, t141746: f64, t141752: f64, t141902: f64, t141989: f64, t150120: f64, t151461: f64, t151471: f64, t1901: f64, t24737: f64, t2599: f64, t27983: f64, t28108: f64, t28140: f64, t28299: f64, t28300: f64, t28364: f64, t33476: f64, t33715: f64, t33760: f64, t35553: f64, t35562: f64, t3837: f64, t3842: f64, t3859: f64, t3864: f64, t3880: f64, t684: f64) -> f64 {
    let t151483 = t8392 * t35621;
    let t151493 = 2.0_f64 / 27.0_f64 * t141722 - 2.0_f64 * t1901 * t28140 * t33715 * t3837 - 2.0_f64 / 3.0_f64 * t1901 * t13885 * t141989 * t3842 - 2.0_f64 / 9.0_f64 * t1901 * t13839 * t33760 + t1901 * t10085 * t35562 / 9.0_f64 + t1901 * t2599 * t141902 * t1091 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t141744 + 4.0_f64 / 9.0_f64 * t141746 - 2.0_f64 / 9.0_f64 * t1901 * t10007 * t35553 * t684 - 4.0_f64 / 3.0_f64 * t1901 * t13885 * t24737 * t28108 - 2.0_f64 / 27.0_f64 * t151461 - 4.0_f64 * t1901 * t28299 * t28300 * t27983 + 2.0_f64 * t1901 * t28140 * t141713 * t3859 - 2.0_f64 / 3.0_f64 * t1901 * t14127 * t151471 * t3864 - 2.0_f64 / 3.0_f64 * t1901 * t13885 * t141989 * t3859 - 4.0_f64 / 3.0_f64 * t1901 * t110401 * t28364 - t151483 / 27.0_f64 - 2.0_f64 / 27.0_f64 * t141752 - 2.0_f64 / 9.0_f64 * t1901 * t14163 * t150120 - t1901 * t10007 * t33476 * t3880 / 9.0_f64;
    t151493
}
