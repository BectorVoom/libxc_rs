//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1075/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1075(t1882: f64, t35578: f64, t35710: f64, t35563: f64, t8392: f64, t10007: f64, t1091: f64, t13885: f64, t13927: f64, t14127: f64, t141873: f64, t1456: f64, t149800: f64, t149899: f64, t149950: f64, t149965: f64, t149967: f64, t151022: f64, t1901: f64, t242: f64, t24737: f64, t2574: f64, t265: f64, t27742: f64, t27924: f64, t28128: f64, t28246: f64, t28276: f64, t33671: f64, t33728: f64, t33782: f64, t35353: f64, t35547: f64, t446: f64, t6154: f64, t6194: f64, t6837: f64, t713: f64, t729: f64, t773: f64) -> f64 {
    let t151787 = t1882 * t35578;
    let t151794 = t1882 * t35710;
    let t151807 = t8392 * t35563;
    let t151827 = -2.0_f64 / 9.0_f64 * t1901 * t10007 * t33728 * t1091 - 2.0_f64 / 9.0_f64 * t1901 * t10007 * t33782 * t1091 + 2.0_f64 / 3.0_f64 * t446 * t2574 * t773 * t35353 + 2.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t151022 - 2.0_f64 * t446 * t242 * t149965 + 4.0_f64 / 3.0_f64 * t446 * t242 * t149967 - 2.0_f64 / 3.0_f64 * t446 * t729 * t13927 * t33671 - 2.0_f64 / 9.0_f64 * t151787 + 2.0_f64 / 3.0_f64 * t446 * t729 * t6154 * t28276 + t141873 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t151794 - 2.0_f64 / 3.0_f64 * t446 * t729 * t1456 * t27742 + 4.0_f64 / 3.0_f64 * t446 * t242 * t149800 - 4.0_f64 / 3.0_f64 * t1901 * t14127 * t28128 * t27924 - t151807 / 27.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t13885 * t24737 * t28246 - t446 * t729 * t35547 * t713 / 3.0_f64 - t446 * t242 * t149950 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t242 * t149899 - 2.0_f64 / 3.0_f64 * t446 * t729 * t6194 * t6837;
    t151827
}
