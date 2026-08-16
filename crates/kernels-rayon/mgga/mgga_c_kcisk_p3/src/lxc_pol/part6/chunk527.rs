//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 527/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk527(t2477: f64, t696: f64, t1814: f64, t2063: f64, t1806: f64, t2488: f64, t2487: f64, t5101: f64, t2365: f64, t821: f64) -> (f64, f64, f64, f64, f64) {
    let t6729 = t696 * t2477;
    let t6734 = t1814 * t2063;
    let t6741 = t1806 * t2488;
    let t6746 = t5101 * t2487;
    let t6756 = t821 * t2365;
    (t6729, t6734, t6741, t6746, t6756)
}
