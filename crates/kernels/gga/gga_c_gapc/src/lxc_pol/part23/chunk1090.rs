//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1090/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1090<F: Float>(t1084: F, t291: F, t33521: F, t33527: F, t4052: F, t3095: F, t6182: F, t9438: F, t33487: F, t33492: F, t33495: F, t33501: F, t33505: F, t33507: F, t33510: F, t33513: F, t33518: F) -> (F, F) {
    let t33528 = t1084 * t4052 * t33521 * t291 * t33527;
    let t33530 = t3095 * t291;
    let t33532 = t9438 * t33530 * t6182;
    let t33534 = -F::new(0.687148483626368822e-6) * t33487 - F::new(0.12290803273518880209e-7) * t33492 + F::new(0.6670285450542344196e-8) * t33495 - F::new(0.13097074855481695406e-9) * t33501 + F::new(0.12290803273518880209e-8) * t33505 + F::new(0.33816362383187442026e-5) * t33507 - F::new(0.31675337336021900772e-5) * t33510 - F::new(0.24760339692676868218e-5) * t33513 + F::new(0.4834058140556728127e-8) * t33518 + F::new(0.14099336243290457037e-8) * t33528 - F::new(0.28960308421505737848e-5) * t33532;
    (t33530, t33534)
}
