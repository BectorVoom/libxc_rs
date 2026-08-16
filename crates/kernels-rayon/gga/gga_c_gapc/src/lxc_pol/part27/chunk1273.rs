//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1273/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1273(t2941: f64, t3638: f64, t3954: f64, t3949: f64, t8459: f64, t11239: f64, t1476: f64, t11512: f64, t14541: f64, t1459: f64, t1649: f64, t3635: f64, t8419: f64) -> (f64, f64, f64, f64, f64) {
    let t35697 = t2941 * t3638 * t3954;
    let t35700 = t8459 * t3638 * t3949;
    let t35702 = t1476 * t11239;
    let t35706 = t14541 * t1459 * t11512 * t1649;
    let t35708 = t8419 * t3635;
    (t35697, t35700, t35702, t35706, t35708)
}
