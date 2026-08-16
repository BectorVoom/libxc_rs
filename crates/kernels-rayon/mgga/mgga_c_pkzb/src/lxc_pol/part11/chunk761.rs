//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 761/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk761(t326: f64, t6455: f64, t401: f64, t5722: f64, t46: f64, t394: f64, t5728: f64, t5939: f64, t922: f64, t918: f64, t54: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6456 = t6455 * t326;
    let t6457 = t401 * t5722;
    let t6458 = t6457 * t46;
    let t6459 = t6456 * t6458;
    let t6462 = t5728 * t394;
    let t6467 = t5939 * t922;
    let t6468 = t918 * t6467;
    let t6475 = t54 * t931;
    (t6456, t6457, t6458, t6459, t6462, t6467, t6468, t6475)
}
