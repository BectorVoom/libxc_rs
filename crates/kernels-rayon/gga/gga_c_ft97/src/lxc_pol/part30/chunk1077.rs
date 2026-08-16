//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1077/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1077(t1882: f64, t35703: f64, t35694: f64, t35717: f64, t35707: f64, t13885: f64, t14127: f64, t141868: f64, t142002: f64, t151079: f64, t151357: f64, t151409: f64, t1901: f64, t242: f64, t28267: f64, t28349: f64, t33759: f64, t3837: f64, t3842: f64, t4005: f64, t446: f64, t53797: f64, t6061: f64, t6154: f64, t6940: f64, t729: f64, t7484: f64, t762: f64, t98123: f64) -> (f64, f64, f64, f64) {
    let t151897 = t1882 * t35703;
    let t151907 = t1882 * t35694;
    let t151926 = t1882 * t35717;
    let t151954 = t1882 * t35707;
    let t151964 = -t446 * t729 * t4005 * t7484 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t242 * t151409 + 2.0_f64 / 3.0_f64 * t446 * t729 * t6154 * t28267 + 2.0_f64 / 3.0_f64 * t446 * t729 * t762 * t6061 * t6940 + 4.0_f64 / 3.0_f64 * t446 * t242 * t151079 + 4.0_f64 / 9.0_f64 * t53797 * t98123 * t28349 - t142002 - t446 * t242 * t151357 / 3.0_f64 + t151954 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t1901 * t13885 * t33759 * t3837 + 2.0_f64 * t1901 * t14127 * t141868 * t3842;
    (t151897, t151907, t151926, t151964)
}
