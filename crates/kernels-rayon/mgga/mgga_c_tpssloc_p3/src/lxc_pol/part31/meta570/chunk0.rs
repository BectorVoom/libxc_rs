//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1802/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1802(t25064: f64, t81902: f64, t23077: f64, t6646: f64, t6590: f64, t23033: f64, t25155: f64, t6546: f64, t25112: f64, t81835: f64, t23083: f64, t25116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87445 = t81902 * t25064;
    let t87447 = t23077 * t6646;
    let t87451 = t6590 * t6646;
    let t87463 = t6546 * t23033 * t25155;
    let t87477 = t81835 * t25112;
    let t87487 = t23083 * t25116;
    (t87445, t87447, t87451, t87463, t87477, t87487)
}
