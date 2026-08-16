//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 739/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk739(t1929: f64, t1932: f64, t1934: f64, t1933: f64, t40: f64, t1937: f64, t3: f64, t607: f64, rho0: f64) -> (f64, f64, f64, f64, f64) {
    let t6720 = t1929 * rho0;
    let t6721 = 1.0_f64 / t6720;
    let t6722 = t6721 * t1932;
    let t6723 = t6722 * t1934;
    let t6726 = t1933 * t40;
    let t6728 = 0.10093189023535097714e-3_f64 * t6726 * t1937;
    let t6729 = t3 * t607;
    (t6721, t6722, t6723, t6728, t6729)
}
