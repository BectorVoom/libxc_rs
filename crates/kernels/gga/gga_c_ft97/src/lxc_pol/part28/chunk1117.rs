//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1117/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1117<F: Float>(t1349: F, t34802: F, t376: F, t34853: F, t379: F, t32979: F, t3424: F, t358: F, t7400: F, t107284: F, t11593: F, t12703: F, t12968: F, t13140: F, t13220: F, t139634: F, t139666: F, t140137: F, t140419: F, t1901: F, t23443: F, t23470: F, t23548: F, t26918: F, t26995: F, t27245: F, t27252: F, t27334: F, t3429: F, t3430: F, t3435: F, t3478: F, t3483: F, t35079: F, t35125: F, t35201: F, t40911: F, t50268: F, t50558: F, t5968: F, t6699: F, t7407: F, t9144: F, t920: F, t9438: F, t95813: F) -> (F, F, F, F) {
    let t147645 = t1349 * t376 * t34802;
    let t147647 = t34853 * t379;
    let t147656 = t32979 * t3424;
    let t147674 = t7400 * t358;
    let t147717 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t12703 * t147647 - t1901 * t9144 * t35125 * t379 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t139634 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t12703 * t147656 - t1901 * t9144 * t32979 * t3429 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t107284 * t27252 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t23470 * t26918 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t40911 * t35201 * t379 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t40911 * t147674 * t3424 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t50558 * t147674 * t3429 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t13220 * t23548 * t920 * t5968 - t139666 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t13140 * t140419 * t3478 - F::cast_from(2.0_f64) * t1901 * t27334 * t9438 * t7407 * t3483 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t23470 * t27245 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t23443 * t26995 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t50268 * t35079 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t12968 * t95813 * t6699 + t1901 * t140137 * t3430 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t140137 * t3435;
    (t147645, t147647, t147656, t147717)
}
