//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1079/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1079(t1882: f64, t34657: f64, t34776: f64, t1852: f64, t5743: f64, t6557: f64, t3255: f64, t7274: f64, t8418: f64, t102848: f64, t11863: f64, t11906: f64, t137768: f64, t137802: f64, t144643: f64, t144701: f64, t144792: f64, t144801: f64, t1901: f64, t25598: f64, t26372: f64, t26373: f64, t26445: f64, t32325: f64, t3238: f64, t32457: f64, t32489: f64, t32495: f64, t32620: f64, t34740: f64, t379: f64, t39120: f64, t446: f64, t452: f64, t47273: f64, t488: f64, t5631: f64, t7211: f64, t83: f64, t91771: f64, t942: f64, t986: f64) -> (f64, f64, f64) {
    let t146088 = t1882 * t34657;
    let t146090 = t1882 * t34776;
    let t146093 = t1852 * t5743 * t6557;
    let t146116 = t8418 * t7274 * t3255;
    let t146129 = -4.0_f64 * t1901 * t26372 * t26373 * t25598 - 2.0_f64 / 27.0_f64 * t137768 - 2.0_f64 / 9.0_f64 * t1901 * t11906 * t32489 - 2.0_f64 / 9.0_f64 * t1901 * t91771 * t26445 + 2.0_f64 / 9.0_f64 * t1901 * t39120 * t34740 * t379 - t446 * t452 * t986 * t32325 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t102848 * t5631 - 2.0_f64 / 9.0_f64 * t1901 * t47273 * t32495 + t146088 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t146090 + 4.0_f64 / 3.0_f64 * t446 * t83 * t146093 + 2.0_f64 / 3.0_f64 * t446 * t452 * t3238 * t32620 - 2.0_f64 / 3.0_f64 * t446 * t83 * t144643 + t446 * t452 * t488 * t7211 * t3255 / 3.0_f64 + t446 * t452 * t488 * t32457 * t942 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t137802 - 2.0_f64 * t446 * t83 * t146116 + 2.0_f64 / 3.0_f64 * t446 * t83 * t144701 - 4.0_f64 / 9.0_f64 * t1901 * t11863 * t144801 + 4.0_f64 / 9.0_f64 * t1901 * t11863 * t144792;
    (t146093, t146116, t146129)
}
