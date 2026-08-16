//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1122/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1122(t35196: f64, t9276: f64, t35193: f64, t8392: f64, t1882: f64, t35157: f64, t35076: f64, t35080: f64, t1060: f64, t106761: f64, t106875: f64, t12664: f64, t13140: f64, t1384: f64, t1391: f64, t139940: f64, t140068: f64, t140078: f64, t144: f64, t1901: f64, t2142: f64, t2185: f64, t23478: f64, t26768: f64, t26849: f64, t26950: f64, t32907: f64, t33080: f64, t35090: f64, t35155: f64, t35201: f64, t446: f64, t574: f64, t5856: f64, t5860: f64, t5935: f64, t605: f64, t6639: f64, t6708: f64, t6725: f64, t9099: f64, t9432: f64) -> (f64, f64) {
    let t147993 = t9276 * t35196;
    let t148030 = t8392 * t35193;
    let t148032 = t1882 * t35157;
    let t148038 = t8392 * t35076;
    let t148046 = t8392 * t35080;
    let t148051 = -2.0_f64 / 3.0_f64 * t446 * t574 * t12664 * t33080 + 2.0_f64 / 3.0_f64 * t446 * t144 * t147993 - 2.0_f64 / 3.0_f64 * t446 * t574 * t9276 * t35201 + 2.0_f64 / 3.0_f64 * t446 * t574 * t2142 * t35155 + 2.0_f64 / 3.0_f64 * t446 * t574 * t605 * t26768 * t1384 + 2.0_f64 / 3.0_f64 * t446 * t574 * t23478 * t6639 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t1391 * t26950 - 2.0_f64 * t446 * t9432 * t1060 * t32907 + 2.0_f64 / 9.0_f64 * t1901 * t106875 * t5856 + 2.0_f64 / 27.0_f64 * t139940 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t6725 * t5860 + 2.0_f64 / 27.0_f64 * t148030 - 2.0_f64 / 9.0_f64 * t148032 + 2.0_f64 / 3.0_f64 * t446 * t574 * t5935 * t26849 - 2.0_f64 / 27.0_f64 * t148038 - t140068 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t140078 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t106761 * t6708 + 4.0_f64 / 9.0_f64 * t148046 + t1901 * t9099 * t35090 / 9.0_f64;
    (t147993, t148051)
}
