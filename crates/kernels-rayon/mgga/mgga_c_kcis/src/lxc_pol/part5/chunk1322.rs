//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1322/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1322(t2001: f64, t5627: f64, t1396: f64, t4123: f64, t1464: f64, t4142: f64, t6914: f64, t3738: f64, t6908: f64, t1394: f64, t556: f64, t7052: f64) -> (f64, f64, f64, f64, f64) {
    let t21876 = t2001 * t5627;
    let t21877 = t1396 * t21876;
    let t21878 = t4123 * t21877;
    let t21879 = t1464 * t21878;
    let t21881 = t4142 * t6914;
    let t21883 = t3738 * t6908;
    let t21884 = t1394 * t21883;
    let t21886 = t7052 * t556;
    (t21876, t21879, t21881, t21884, t21886)
}
