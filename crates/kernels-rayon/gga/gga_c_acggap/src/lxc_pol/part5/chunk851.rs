//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 851/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk851(t11947: f64, t689: f64, t11870: f64, t286: f64, t11545: f64, t11552: f64, t11560: f64, t11566: f64, t11574: f64, t11578: f64, t123: f64, t132: f64, t265: f64, t2696: f64, t2700: f64, t2718: f64, t2723: f64, t273: f64, t2743: f64, t2755: f64, t2759: f64, t2763: f64, t2767: f64, t2769: f64, t2773: f64, t2776: f64, t2813: f64, t328: f64, t686: f64, t721: f64, t722: f64, t740: f64, t791: f64, t793: f64, t796: f64, t800: f64) -> (f64, f64, f64, f64) {
    let t11948 = 1.0_f64 / t11947;
    let t11950 = t689 * t689;
    let t11951 = 1.0_f64 / t11950;
    let t11954 = 0.91082604192152556044e5_f64 * t286 * t11948 * t11870 * t11951;
    let t11995 = -t11545 + t11552 + t11560 - t11566 - 0.41096e0_f64 * t721 * t2718 * t2743 + 0.38527786510141256862e1_f64 * t721 * t132 * t2773 * t2776 + 0.13218100589565368422e2_f64 * t721 * t132 * t2767 * t2769 - 0.68493333333333333332e-1_f64 * t721 * t722 * t2755 - 0.14171548179536397724e3_f64 * t721 * t132 * t2759 * t2763 + 0.13698666666666666666e0_f64 * t721 * t2723 * t740 + 0.43374325201206959368e-1_f64 * t721 * t2700 * t796 - 0.1301229756036208781e0_f64 * t721 * t2696 * t2813 + t11574 - t11578 + 0.12842595503380418954e1_f64 * t721 * t123 * t686 * t800 - 0.67471172535210825684e-1_f64 * t721 * t328 * t265 * t273 - 0.86748650402413918736e-1_f64 * t721 * t123 * t791 * t793;
    (t11948, t11951, t11954, t11995)
}
