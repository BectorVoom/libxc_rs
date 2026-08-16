//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2030/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2030(t22690: f64, t2379: f64, t81792: f64, t841: f64, t23072: f64, t23083: f64, t23069: f64, t2610: f64, t2690: f64, t6612: f64, t812: f64, t831: f64) -> (f64, f64, f64, f64, f64) {
    let t81795 = t81792 * t22690 * t841 * t2379;
    let t81797 = t23083 * t23072;
    let t81799 = t23069 * t2610;
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    (t81795, t81797, t81799, t81807, t81808)
}
