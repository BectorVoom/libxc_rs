//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 777/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk777(t2719: f64, t788: f64, t2201: f64, t785: f64, t2202: f64, t2837: f64, t1620: f64, t2682: f64, t129: f64, t1598: f64, t524: f64, t2593: f64) -> (f64, f64, f64, f64, f64) {
    let t7476 = t788 * t2719;
    let t7479 = 0.11643651550782197811e-1_f64 * t2201 * t785 * t7476;
    let t7482 = 0.11643651550782197811e-1_f64 * t2201 * t2837 * t2202;
    let t7490 = t1620 * t2682;
    let t7494 = t524 * t1598 * t129;
    let t7496 = 0.25610080155860322884e0_f64 * t7494 * t2593;
    (t7479, t7482, t7490, t7494, t7496)
}
