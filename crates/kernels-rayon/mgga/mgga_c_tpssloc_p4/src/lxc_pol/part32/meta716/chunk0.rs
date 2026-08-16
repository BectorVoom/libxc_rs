//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2259/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2259(t849: f64, t98832: f64, t23083: f64, t28375: f64, t28396: f64, t81835: f64, t58853: f64, t6605: f64, t828: f64, t9972: f64, t4250: f64, t87199: f64) -> (f64, f64, f64, f64, f64) {
    let t98833 = t98832 * t849;
    let t98836 = t23083 * t28375;
    let t98838 = t81835 * t28396;
    let t98842 = t6605 * t9972 * t58853 * t828;
    let t98844 = t87199 * t4250;
    (t98833, t98836, t98838, t98842, t98844)
}
