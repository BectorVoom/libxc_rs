//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1142/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1142(t3327: f64, t33655: f64, t33685: f64, t7073: f64, t3751: f64, t9635: f64, t11954: f64, t3392: f64, t11957: f64, t2387: f64, t3297: f64, t3761: f64) -> (f64, f64, f64, f64, f64) {
    let t34142 = t7073 * t33655 * t3327 * t33685;
    let t34144 = t3751 * t9635;
    let t34146 = t11954 * t3392;
    let t34148 = t11957 * t3392;
    let t34151 = t2387 * t3761 * t3297;
    (t34142, t34144, t34146, t34148, t34151)
}
