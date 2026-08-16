//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 842/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk842(t9775: f64, t9777: f64, t9780: f64, t9783: f64, t9789: f64, t9791: f64, t9793: f64, t9796: f64, t9800: f64, t9802: f64, t9805: f64, t9808: f64, t9811: f64) -> f64 {
    let t9813 = 0.61644410594352107859e-7_f64 * t9775 + 0.4637672555408563478e-4_f64 * t9777 + 0.38647271295071362318e-6_f64 * t9780 - 0.687148483626368822e-6_f64 * t9783 - 0.2813674965076916843e-8_f64 * t9789 - 0.4637672555408563478e-4_f64 * t9791 + 0.66340671383216596998e-6_f64 * t9793 - 0.27801896084645508334e-2_f64 * t9796 - 0.14758978949652777778e-5_f64 * t9800 - 0.13492128925537291361e-5_f64 * t9802 - 0.7588373973867992891e-7_f64 * t9805 + 0.13492128925537291361e-6_f64 * t9808 - 0.28985453471303521736e-5_f64 * t9811;
    t9813
}
