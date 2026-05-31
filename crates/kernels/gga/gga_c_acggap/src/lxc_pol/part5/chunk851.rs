//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 851/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk851<F: Float>(t11947: F, t689: F, t11870: F, t286: F, t11545: F, t11552: F, t11560: F, t11566: F, t11574: F, t11578: F, t123: F, t132: F, t265: F, t2696: F, t2700: F, t2718: F, t2723: F, t273: F, t2743: F, t2755: F, t2759: F, t2763: F, t2767: F, t2769: F, t2773: F, t2776: F, t2813: F, t328: F, t686: F, t721: F, t722: F, t740: F, t791: F, t793: F, t796: F, t800: F) -> (F, F, F, F) {
    let t11948 = F::cast_from(1.0_f64) / t11947;
    let t11950 = t689 * t689;
    let t11951 = F::cast_from(1.0_f64) / t11950;
    let t11954 = F::cast_from(0.91082604192152556044e5_f64) * t286 * t11948 * t11870 * t11951;
    let t11995 = -t11545 + t11552 + t11560 - t11566 - F::cast_from(0.41096e0_f64) * t721 * t2718 * t2743 + F::cast_from(0.38527786510141256862e1_f64) * t721 * t132 * t2773 * t2776 + F::cast_from(0.13218100589565368422e2_f64) * t721 * t132 * t2767 * t2769 - F::cast_from(0.68493333333333333332e-1_f64) * t721 * t722 * t2755 - F::cast_from(0.14171548179536397724e3_f64) * t721 * t132 * t2759 * t2763 + F::cast_from(0.13698666666666666666e0_f64) * t721 * t2723 * t740 + F::cast_from(0.43374325201206959368e-1_f64) * t721 * t2700 * t796 - F::cast_from(0.1301229756036208781e0_f64) * t721 * t2696 * t2813 + t11574 - t11578 + F::cast_from(0.12842595503380418954e1_f64) * t721 * t123 * t686 * t800 - F::cast_from(0.67471172535210825684e-1_f64) * t721 * t328 * t265 * t273 - F::cast_from(0.86748650402413918736e-1_f64) * t721 * t123 * t791 * t793;
    (t11948, t11951, t11954, t11995)
}
