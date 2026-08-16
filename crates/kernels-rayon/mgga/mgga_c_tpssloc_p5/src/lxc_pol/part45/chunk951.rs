//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 951/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk951(t1307: f64, t1352: f64, t2006: f64, t22633: f64, t6976: f64, t22751: f64, t31195: f64, t22892: f64, t22893: f64, t31194: f64, t22642: f64, t22690: f64, t31193: f64) -> (f64, f64, f64, f64) {
    let t114056 = 0.6579736267392905746e-1_f64 * t22633 * t6976 * t2006 * t1307 * t1352;
    let t114057 = t22751 * t31195;
    let t114058 = 0.15352717957250113407e0_f64 * t114057;
    let t114060 = t22892 * t22893 * t31194;
    let t114061 = 0.3289868133696452873e-1_f64 * t114060;
    let t114064 = 0.16449340668482264365e-1_f64 * t22642 * t22690 * t31193;
    (t114056, t114058, t114061, t114064)
}
