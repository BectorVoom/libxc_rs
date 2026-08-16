//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1061/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1061(t11748: f64, t2600: f64, t11804: f64, t11814: f64, t2599: f64, t11325: f64, t3402: f64, t9934: f64, t11872: f64, t9723: f64, t33179: f64, t33182: f64, t33185: f64, t33187: f64, t33190: f64, t33193: f64, t33195: f64) -> (f64, f64) {
    let t33197 = t11748 * t2600;
    let t33200 = t11814 * t11804 * t2599;
    let t33202 = t3402 * t11325;
    let t33203 = t33202 * t9934;
    let t33205 = t11872 * t9723;
    let t33207 = 0.51491428373437201896e-5_f64 * t33179 + 0.687148483626368822e-6_f64 * t33182 - 0.11254699860307667372e-7_f64 * t33185 - 0.16573913624765925007e-7_f64 * t33187 - 0.22509399720615334744e-7_f64 * t33190 + 0.16908181191593721013e-5_f64 * t33193 + 0.4637672555408563478e-4_f64 * t33195 - 0.4637672555408563478e-4_f64 * t33197 - 0.24581606547037760418e-8_f64 * t33200 + 0.32042899674547455014e-6_f64 * t33203 + 0.33764099580923002116e-6_f64 * t33205;
    (t33202, t33207)
}
