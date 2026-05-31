//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1122/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1122<F: Float>(t35196: F, t9276: F, t35193: F, t8392: F, t1882: F, t35157: F, t35076: F, t35080: F, t1060: F, t106761: F, t106875: F, t12664: F, t13140: F, t1384: F, t1391: F, t139940: F, t140068: F, t140078: F, t144: F, t1901: F, t2142: F, t2185: F, t23478: F, t26768: F, t26849: F, t26950: F, t32907: F, t33080: F, t35090: F, t35155: F, t35201: F, t446: F, t574: F, t5856: F, t5860: F, t5935: F, t605: F, t6639: F, t6708: F, t6725: F, t9099: F, t9432: F) -> (F, F) {
    let t147993 = t9276 * t35196;
    let t148030 = t8392 * t35193;
    let t148032 = t1882 * t35157;
    let t148038 = t8392 * t35076;
    let t148046 = t8392 * t35080;
    let t148051 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t12664 * t33080 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t147993 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t9276 * t35201 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t2142 * t35155 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t605 * t26768 * t1384 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t23478 * t6639 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t1391 * t26950 - F::cast_from(2.0_f64) * t446 * t9432 * t1060 * t32907 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t106875 * t5856 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t139940 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t6725 * t5860 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t148030 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148032 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t5935 * t26849 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t148038 - t140068 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140078 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t13140 * t106761 * t6708 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t148046 + t1901 * t9099 * t35090 / F::cast_from(9.0_f64);
    (t147993, t148051)
}
