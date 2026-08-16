//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1071/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1071(t35570: f64, t8392: f64, t35559: f64, t35567: f64, t110019: f64, t13885: f64, t14127: f64, t14163: f64, t141759: f64, t142030: f64, t142347: f64, t1443: f64, t150034: f64, t150064: f64, t1901: f64, t24737: f64, t24789: f64, t2486: f64, t28124: f64, t28153: f64, t28267: f64, t28298: f64, t28301: f64, t33708: f64, t35558: f64, t35609: f64, t35620: f64, t3875: f64, t3881: f64, t3887: f64, t3893: f64, t42339: f64, t42376: f64, t51669: f64, t53923: f64, t53942: f64, t6921: f64, t6930: f64, t7536: f64, t97810: f64, t9787: f64) -> f64 {
    let t151507 = t8392 * t35570;
    let t151533 = t8392 * t35559;
    let t151546 = t8392 * t35567;
    let t151555 = 4.0_f64 / 9.0_f64 * t1901 * t14163 * t150064 - 4.0_f64 / 27.0_f64 * t1901 * t51669 * t150034 - 4.0_f64 / 3.0_f64 * t1901 * t53942 * t35609 - 4.0_f64 / 3.0_f64 * t1901 * t13885 * t97810 * t6921 - 2.0_f64 / 27.0_f64 * t151507 + 2.0_f64 / 27.0_f64 * t141759 + 2.0_f64 / 9.0_f64 * t1901 * t24789 * t28124 + t1901 * t142030 * t3881 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t142030 * t3887 - 2.0_f64 / 27.0_f64 * t1901 * t2486 * t7536 * t3893 - 2.0_f64 / 9.0_f64 * t1901 * t42376 * t35558 - 4.0_f64 / 3.0_f64 * t1901 * t13885 * t24737 * t28267 + t1901 * t9787 * t35620 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t151533 + 2.0_f64 / 9.0_f64 * t1901 * t24789 * t28153 - 4.0_f64 / 3.0_f64 * t1901 * t14127 * t110019 * t6930 - 4.0_f64 * t1901 * t28298 * t1443 * t28301 + 2.0_f64 / 27.0_f64 * t151546 - 2.0_f64 / 9.0_f64 * t1901 * t53923 * t33708 + 2.0_f64 / 9.0_f64 * t1901 * t42339 * t142347 * t3875;
    t151555
}
