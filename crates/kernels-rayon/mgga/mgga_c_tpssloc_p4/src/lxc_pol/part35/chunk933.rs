//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 933/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk933(t1358: f64, t6379: f64, t12211: f64, t6371: f64, t3726: f64, t6375: f64, t12385: f64, t6390: f64, t16288: f64, t1827: f64, t1340: f64, t19815: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19834 = t6379 * t1358;
    let t19839 = t12211 * t6371;
    let t19841 = t3726 * t6375;
    let t19851 = t12385 * t6390;
    let t19853 = t16288 * t1827;
    let t19855 = t19815 * t1340;
    (t19834, t19839, t19841, t19851, t19853, t19855)
}
