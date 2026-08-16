//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1095/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1095(t1337: f64, t358: f64, t1286: f64, t34584: f64, t376: f64, t137488: f64, t137547: f64, t146093: f64, t146376: f64, t146561: f64, t1557: f64, t1570: f64, t22907: f64, t25601: f64, t25609: f64, t25615: f64, t25622: f64, t26119: f64, t28: f64, t3188: f64, t32016: f64, t3204: f64, t32641: f64, t432: f64, t5501: f64, t5507: f64, t6562: f64, t7162: f64, t948: f64) -> f64 {
    let t147008 = t1337 * t358;
    let t147024 = t1286 * t376 * t34584;
    let t147040 = 8.0_f64 * t146376 + 8.0_f64 * t146561 + 8.0_f64 * t146093 + 2.0_f64 / 9.0_f64 * t5501 * t22907 * t147008 * t3204 + 2.0_f64 / 9.0_f64 * t5501 * t25609 * t1337 * t1570 * t3188 - 2.0_f64 / 27.0_f64 * t5501 * t25615 * t1337 * t1557 * t3188 - t147024 / 9.0_f64 - t32016 * t26119 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t5507 * t6562 * t432 - t948 * t32641 - t137547 / 18.0_f64 - t5501 * t137488 * t25601 / 3.0_f64 + t7162 * t25622 / 6.0_f64;
    t147040
}
