//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1314/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1314(t3638: f64, t3949: f64, t8459: f64, t11239: f64, t1476: f64, t11512: f64, t14541: f64, t1459: f64, t1649: f64, t3635: f64, t8419: f64, t11683: f64, t22971: f64, t22973: f64, t3737: f64) -> (f64, f64, f64, f64, f64) {
    let t35700 = t8459 * t3638 * t3949;
    let t35702 = t1476 * t11239;
    let t35706 = t14541 * t1459 * t11512 * t1649;
    let t35708 = t8419 * t3635;
    let t35720 = t3737 * t22971 * t11683 * t22973;
    (t35700, t35702, t35706, t35708, t35720)
}
