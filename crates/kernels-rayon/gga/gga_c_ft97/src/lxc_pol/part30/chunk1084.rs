//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1084/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1084(t35614: f64, t8392: f64, t10079: f64, t1091: f64, t109848: f64, t110629: f64, t11593: f64, t124402: f64, t13885: f64, t14127: f64, t14187: f64, t142333: f64, t142334: f64, t142365: f64, t142382: f64, t142393: f64, t142395: f64, t149867: f64, t150042: f64, t151353: f64, t151430: f64, t1901: f64, t2347: f64, t242: f64, t2469: f64, t33532: f64, t33771: f64, t33772: f64, t35634: f64, t3746: f64, t3859: f64, t3864: f64, t3886: f64, t446: f64, t52006: f64, t53662: f64, t6166: f64, t6175: f64, t67996: f64, t724: f64, t729: f64, t7502: f64, t7553: f64) -> f64 {
    let t152361 = t8392 * t35614;
    let t152404 = t446 * t729 * t2469 * t35634 / 3.0_f64 - t446 * t242 * t151353 / 3.0_f64 + t142333 - t446 * t724 * t33532 * t1091 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t142334 + 4.0_f64 / 9.0_f64 * t152361 - t142365 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t142382 + 4.0_f64 / 9.0_f64 * t11593 * t10079 * t33771 * t3746 - 2.0_f64 / 9.0_f64 * t1901 * t52006 * t33772 - 4.0_f64 / 3.0_f64 * t1901 * t13885 * t110629 * t6166 - 4.0_f64 / 3.0_f64 * t1901 * t14127 * t124402 * t6175 + 2.0_f64 / 3.0_f64 * t1901 * t53662 * t150042 + 2.0_f64 / 9.0_f64 * t142393 - 4.0_f64 / 9.0_f64 * t142395 - 2.0_f64 / 3.0_f64 * t446 * t242 * t151430 + 4.0_f64 / 3.0_f64 * t446 * t242 * t149867 + 8.0_f64 / 3.0_f64 * t1901 * t67996 * t7502 * t3859 + 4.0_f64 * t1901 * t109848 * t7502 * t3864 + 2.0_f64 / 27.0_f64 * t1901 * t14187 * t7553 * t2347 * t3886;
    t152404
}
