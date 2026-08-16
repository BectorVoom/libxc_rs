//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1170/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1170(t1086: f64, t11986: f64, t22783: f64, t11311: f64, t11987: f64, t8117: f64, t11483: f64, t15843: f64, t2597: f64, t2675: f64, t189: f64, t615: f64) -> (f64, f64, f64, f64) {
    let t33634 = t11986 * t1086 * t22783;
    let t33637 = t8117 * t11311 * t11987;
    let t33641 = t2675 * t11483 * t2597 * t15843;
    let t33643 = t189 * t615;
    (t33634, t33637, t33641, t33643)
}
