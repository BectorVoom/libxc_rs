//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1358/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1358(t31051: f64, t7458: f64, t2314: f64, t32663: f64, t4034: f64, t1873: f64, t25958: f64, t652: f64, t1874: f64, t96361: f64, t24999: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120740 = t7458 * t31051;
    let t120742 = t2314 * t32663;
    let t120744 = t4034 * t32663;
    let t120747 = t652 * t25958 * t1873;
    let t120749 = t96361 * t1874;
    let t120751 = t24999 * t6525;
    (t120740, t120742, t120744, t120747, t120749, t120751)
}
