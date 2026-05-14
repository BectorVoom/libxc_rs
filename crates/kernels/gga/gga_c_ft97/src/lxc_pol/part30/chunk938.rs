//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 938/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk938<F: Float>(t35570: F, t8392: F, t35559: F, t35567: F, t110019: F, t13885: F, t14127: F, t14163: F, t141759: F, t142030: F, t142347: F, t1443: F, t150034: F, t150064: F, t1901: F, t24737: F, t24789: F, t2486: F, t28124: F, t28153: F, t28267: F, t28298: F, t28301: F, t33708: F, t35558: F, t35609: F, t35620: F, t3875: F, t3881: F, t3887: F, t3893: F, t42339: F, t42376: F, t51669: F, t53923: F, t53942: F, t6921: F, t6930: F, t7536: F, t97810: F, t9787: F) -> (F,) {
    let t151507 = t8392 * t35570;
    let t151533 = t8392 * t35559;
    let t151546 = t8392 * t35567;
    let t151555 = 4.0 / 9.0 * t1901 * t14163 * t150064 - 4.0 / 27.0 * t1901 * t51669 * t150034 - 4.0 / 3.0 * t1901 * t53942 * t35609 - 4.0 / 3.0 * t1901 * t13885 * t97810 * t6921 - 2.0 / 27.0 * t151507 + 2.0 / 27.0 * t141759 + 2.0 / 9.0 * t1901 * t24789 * t28124 + t1901 * t142030 * t3881 / 9.0 + 2.0 / 9.0 * t1901 * t142030 * t3887 - 2.0 / 27.0 * t1901 * t2486 * t7536 * t3893 - 2.0 / 9.0 * t1901 * t42376 * t35558 - 4.0 / 3.0 * t1901 * t13885 * t24737 * t28267 + t1901 * t9787 * t35620 / 9.0 + 2.0 / 27.0 * t151533 + 2.0 / 9.0 * t1901 * t24789 * t28153 - 4.0 / 3.0 * t1901 * t14127 * t110019 * t6930 - 4.0 * t1901 * t28298 * t1443 * t28301 + 2.0 / 27.0 * t151546 - 2.0 / 9.0 * t1901 * t53923 * t33708 + 2.0 / 9.0 * t1901 * t42339 * t142347 * t3875;
    (t151555,)
}
