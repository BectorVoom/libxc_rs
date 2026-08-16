//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1119/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1119(t35064: f64, t8392: f64, t160: f64, t34918: f64, t1882: f64, t35060: f64, t35127: f64, t35091: f64, t2178: f64, t7339: f64, t107082: f64, t11593: f64, t12968: f64, t13140: f64, t1378: f64, t139757: f64, t139767: f64, t139791: f64, t139950: f64, t1901: f64, t2210: f64, t2221: f64, t23455: f64, t23571: f64, t26520: f64, t26849: f64, t26897: f64, t26924: f64, t27015: f64, t27333: f64, t27336: f64, t3052: f64, t33034: f64, t3455: f64, t3478: f64, t3483: f64, t35094: f64, t379: f64, t446: f64, t574: f64, t5968: f64, t605: f64, t6615: f64, t6626: f64, t9099: f64, t95696: f64) -> f64 {
    let t147797 = t8392 * t35064;
    let t147806 = t160 * t34918;
    let t147830 = t1882 * t35060;
    let t147837 = t1882 * t35127;
    let t147839 = t8392 * t35091;
    let t147845 = t2178 * t7339;
    let t147855 = 2.0_f64 / 9.0_f64 * t1901 * t95696 * t6626 - 4.0_f64 * t1901 * t27333 * t1378 * t27336 - t147797 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t9099 * t35094 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t23455 * t26849 + t1901 * t2221 * t147806 * t379 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t107082 * t26924 - 4.0_f64 / 3.0_f64 * t1901 * t12968 * t23571 * t26897 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t27015 * t26520 + 2.0_f64 / 9.0_f64 * t11593 * t2210 * t33034 * t3052 + 2.0_f64 * t1901 * t13140 * t139757 * t3455 - t139767 - 2.0_f64 / 9.0_f64 * t147830 + 2.0_f64 / 3.0_f64 * t446 * t574 * t605 * t6615 * t5968 - t147837 / 9.0_f64 - t147839 / 27.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t12968 * t139950 * t3478 - 2.0_f64 / 3.0_f64 * t1901 * t13140 * t147845 * t3483 - 2.0_f64 / 3.0_f64 * t1901 * t12968 * t139950 * t3455 + 2.0_f64 / 9.0_f64 * t139791;
    t147855
}
