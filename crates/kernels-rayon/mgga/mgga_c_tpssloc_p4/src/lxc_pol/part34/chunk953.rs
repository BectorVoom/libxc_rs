//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 953/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk953(t21898: f64, t21991: f64, t300: f64, t1763: f64, t6274: f64, t11947: f64, t193: f64, t21726: f64, t21728: f64, t21730: f64, t21732: f64, t21812: f64, t21815: f64, t21829: f64, t21832: f64, t21835: f64, t21897: f64, t21901: f64, t336: f64) -> (f64, f64) {
    let t21993 = t300 * (t21898 + t21991);
    let t21994 = t6274 * t1763;
    let t21999 = 2.0_f64 * t11947 * t193 * t21994 * t336 + t21726 - t21728 - t21730 + t21732 + t21812 + t21815 + t21829 - t21832 + t21835 - t21897 + t21901 + t21993;
    (t21993, t21999)
}
