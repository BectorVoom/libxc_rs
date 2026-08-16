//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1106/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1106(t11994: f64, t33696: f64, t33258: f64, t3781: f64, t11356: f64, t9563: f64, t9934: f64, t474: f64, t8837: f64, t10031: f64, t3402: f64, t1084: f64, t9923: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33741 = t33696 * t11994;
    let t33743 = t33258 * t3781;
    let t33746 = t9563 * t11356 * t9934;
    let t33748 = t474 * t8837;
    let t33750 = t3402 * t33748 * t10031;
    let t33753 = t1084 * t33748 * t9923;
    (t33741, t33743, t33746, t33748, t33750, t33753)
}
