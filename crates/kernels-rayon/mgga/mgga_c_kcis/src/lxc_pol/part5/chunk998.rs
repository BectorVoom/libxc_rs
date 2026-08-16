//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 998/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk998(t1627: f64, t629: f64, t2791: f64, t838: f64, t169: f64, t2628: f64, t174: f64, t2640: f64, t2792: f64, t2627: f64, t2540: f64, t2534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12938 = t1627 * t1627;
    let t12939 = 1.0_f64 / t12938;
    let t12940 = t629 * t12939;
    let t13000 = t838 * t2791;
    let t13003 = 1.0_f64 / t2628 / t169;
    let t13014 = 1.0_f64 / t2640 / t174;
    let t13031 = 3.0_f64 * t2792;
    let t13034 = 3.0_f64 * t2627;
    let t13043 = 6.0_f64 * t2540;
    let t13044 = 6.0_f64 * t2534;
    (t12940, t13000, t13003, t13014, t13031, t13034, t13043, t13044)
}
