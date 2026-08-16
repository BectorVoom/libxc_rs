//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1168/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1168(t35149: f64, t604: f64, t609: f64, t1882: f64, t35211: f64, t1060: f64, t12680: f64, t13220: f64, t1359: f64, t139702: f64, t140275: f64, t140278: f64, t140288: f64, t140290: f64, t144: f64, t148451: f64, t148613: f64, t148860: f64, t148880: f64, t148897: f64, t167: f64, t1901: f64, t2142: f64, t2185: f64, t26590: f64, t26897: f64, t27414: f64, t32951: f64, t33056: f64, t3424: f64, t3429: f64, t34822: f64, t35160: f64, t446: f64, t574: f64, t5869: f64, t5935: f64, t616: f64, t9144: f64) -> (f64, f64) {
    let t148905 = t35149 * t604;
    let t148906 = t148905 * t609;
    let t148914 = t1882 * t35211;
    let t148921 = 2.0_f64 / 3.0_f64 * t446 * t574 * t5935 * t26897 - t446 * t144 * t148860 / 3.0_f64 - t1901 * t9144 * t139702 * t3424 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t13220 * t139702 * t3429 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t616 * t34822 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t167 * t148451 + 2.0_f64 / 3.0_f64 * t446 * t144 * t148880 + 2.0_f64 / 3.0_f64 * t446 * t574 * t2142 * t35160 - 2.0_f64 / 27.0_f64 * t140275 + 2.0_f64 / 3.0_f64 * t446 * t2185 * t1060 * t32951 + 2.0_f64 / 3.0_f64 * t446 * t2185 * t167 * t148613 - 2.0_f64 / 3.0_f64 * t446 * t144 * t148897 - 2.0_f64 / 3.0_f64 * t446 * t574 * t27414 * t1359 - t446 * t144 * t148906 / 3.0_f64 + t140278 + 2.0_f64 / 3.0_f64 * t446 * t574 * t26590 * t5869 + 2.0_f64 / 9.0_f64 * t148914 - 4.0_f64 / 9.0_f64 * t140288 + 2.0_f64 / 9.0_f64 * t140290 - 2.0_f64 / 9.0_f64 * t1901 * t12680 * t33056;
    (t148906, t148921)
}
