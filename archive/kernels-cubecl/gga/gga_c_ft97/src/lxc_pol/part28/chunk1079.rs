//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1079/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1079<F: Float>(t1882: F, t34657: F, t34776: F, t1852: F, t5743: F, t6557: F, t3255: F, t7274: F, t8418: F, t102848: F, t11863: F, t11906: F, t137768: F, t137802: F, t144643: F, t144701: F, t144792: F, t144801: F, t1901: F, t25598: F, t26372: F, t26373: F, t26445: F, t32325: F, t3238: F, t32457: F, t32489: F, t32495: F, t32620: F, t34740: F, t379: F, t39120: F, t446: F, t452: F, t47273: F, t488: F, t5631: F, t7211: F, t83: F, t91771: F, t942: F, t986: F) -> (F, F, F) {
    let t146088 = t1882 * t34657;
    let t146090 = t1882 * t34776;
    let t146093 = t1852 * t5743 * t6557;
    let t146116 = t8418 * t7274 * t3255;
    let t146129 = -F::cast_from(4.0_f64) * t1901 * t26372 * t26373 * t25598 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t137768 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11906 * t32489 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t91771 * t26445 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t39120 * t34740 * t379 - t446 * t452 * t986 * t32325 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t102848 * t5631 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t47273 * t32495 + t146088 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146090 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t146093 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t3238 * t32620 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t144643 + t446 * t452 * t488 * t7211 * t3255 / F::cast_from(3.0_f64) + t446 * t452 * t488 * t32457 * t942 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t137802 - F::cast_from(2.0_f64) * t446 * t83 * t146116 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t144701 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11863 * t144801 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11863 * t144792;
    (t146093, t146116, t146129)
}
