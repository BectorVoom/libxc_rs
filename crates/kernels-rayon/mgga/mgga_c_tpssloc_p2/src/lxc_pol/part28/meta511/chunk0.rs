//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1759/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1759(t13030: f64, t225: f64, t13062: f64, t13378: f64, t193: f64, t2379: f64, t16465: f64, t12250: f64, t1824: f64, t1799: f64, t3791: f64, t3850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47585 = t13030 * t225;
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47645 = t193 * t2379;
    let t53866 = t16465 * t225;
    let t54014 = t1824 * t12250;
    let t54068 = t1799 * t3791;
    let t54153 = t1824 * t3850;
    (t47585, t47609, t47618, t47645, t53866, t54014, t54068, t54153)
}
