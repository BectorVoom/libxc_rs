//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1061/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1061<F: Float>(t11748: F, t2600: F, t11804: F, t11814: F, t2599: F, t11325: F, t3402: F, t9934: F, t11872: F, t9723: F, t33179: F, t33182: F, t33185: F, t33187: F, t33190: F, t33193: F, t33195: F) -> (F, F) {
    let t33197 = t11748 * t2600;
    let t33200 = t11814 * t11804 * t2599;
    let t33202 = t3402 * t11325;
    let t33203 = t33202 * t9934;
    let t33205 = t11872 * t9723;
    let t33207 = F::new(0.51491428373437201896e-5) * t33179 + F::new(0.687148483626368822e-6) * t33182 - F::new(0.11254699860307667372e-7) * t33185 - F::new(0.16573913624765925007e-7) * t33187 - F::new(0.22509399720615334744e-7) * t33190 + F::new(0.16908181191593721013e-5) * t33193 + F::new(0.4637672555408563478e-4) * t33195 - F::new(0.4637672555408563478e-4) * t33197 - F::new(0.24581606547037760418e-8) * t33200 + F::new(0.32042899674547455014e-6) * t33203 + F::new(0.33764099580923002116e-6) * t33205;
    (t33202, t33207)
}
