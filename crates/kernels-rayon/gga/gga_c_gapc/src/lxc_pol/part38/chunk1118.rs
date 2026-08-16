//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1118/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1118(t11917: f64, t29481: f64, t3402: f64, t2387: f64, t3750: f64, t3752: f64, t33530: f64, t3430: f64, t6188: f64, t11853: f64, t291: f64, t8685: f64) -> (f64, f64, f64, f64) {
    let t33875 = t3402 * t11917 * t29481;
    let t33878 = t2387 * t3750 * t3752;
    let t33881 = t3430 * t33530 * t6188;
    let t33884 = t8685 * t291 * t11853;
    (t33875, t33878, t33881, t33884)
}
