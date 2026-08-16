//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1117/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1117(t1349: f64, t34802: f64, t376: f64, t34853: f64, t379: f64, t32979: f64, t3424: f64, t358: f64, t7400: f64, t107284: f64, t11593: f64, t12703: f64, t12968: f64, t13140: f64, t13220: f64, t139634: f64, t139666: f64, t140137: f64, t140419: f64, t1901: f64, t23443: f64, t23470: f64, t23548: f64, t26918: f64, t26995: f64, t27245: f64, t27252: f64, t27334: f64, t3429: f64, t3430: f64, t3435: f64, t3478: f64, t3483: f64, t35079: f64, t35125: f64, t35201: f64, t40911: f64, t50268: f64, t50558: f64, t5968: f64, t6699: f64, t7407: f64, t9144: f64, t920: f64, t9438: f64, t95813: f64) -> (f64, f64, f64, f64) {
    let t147645 = t1349 * t376 * t34802;
    let t147647 = t34853 * t379;
    let t147656 = t32979 * t3424;
    let t147674 = t7400 * t358;
    let t147717 = -2.0_f64 / 9.0_f64 * t1901 * t12703 * t147647 - t1901 * t9144 * t35125 * t379 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t139634 - 2.0_f64 / 9.0_f64 * t1901 * t12703 * t147656 - t1901 * t9144 * t32979 * t3429 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t107284 * t27252 + 4.0_f64 / 9.0_f64 * t11593 * t23470 * t26918 + 2.0_f64 / 9.0_f64 * t1901 * t40911 * t35201 * t379 + 2.0_f64 / 9.0_f64 * t1901 * t40911 * t147674 * t3424 + 2.0_f64 / 3.0_f64 * t1901 * t50558 * t147674 * t3429 - 4.0_f64 / 9.0_f64 * t1901 * t13220 * t23548 * t920 * t5968 - t139666 / 27.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t13140 * t140419 * t3478 - 2.0_f64 * t1901 * t27334 * t9438 * t7407 * t3483 + 2.0_f64 / 9.0_f64 * t1901 * t23470 * t27245 + 4.0_f64 / 9.0_f64 * t11593 * t23443 * t26995 - 4.0_f64 / 3.0_f64 * t1901 * t50268 * t35079 - 4.0_f64 / 3.0_f64 * t1901 * t12968 * t95813 * t6699 + t1901 * t140137 * t3430 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t140137 * t3435;
    (t147645, t147647, t147656, t147717)
}
