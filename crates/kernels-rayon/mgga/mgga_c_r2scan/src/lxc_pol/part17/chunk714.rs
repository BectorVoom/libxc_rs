//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 714/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk714(t672: f64, t685: f64, t63: f64, t1664: f64, t1800: f64, t649: f64, t2029: f64, t689: f64, t1663: f64, t5381: f64, t390: f64, t188: f64, t1890: f64) -> (f64, f64, f64, f64, f64) {
    let t5747 = 1.0_f64 / t685 / t672;
    let t5748 = t63 * t5747;
    let t5754 = 0.10310157056611784231e2_f64 * t649 * t1800 * t1664;
    let t5755 = t2029 * t689;
    let t5759 = t1663 * t5381;
    let t5761 = 0.85917975471764868594e0_f64 * t390 * t5759;
    let t5762 = t1890 * t188;
    (t5748, t5754, t5755, t5761, t5762)
}
