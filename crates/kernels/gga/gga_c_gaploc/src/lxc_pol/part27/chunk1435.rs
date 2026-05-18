//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1435/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1435<F: Float>(t1: F, t39048: F, t2021: F, t2610: F, t38912: F, t12205: F, t1858: F, t2005: F, t2028: F, t28714: F, t33501: F, t33505: F, t33508: F, t33518: F, t33522: F, t33526: F, t33529: F, t33532: F, t33536: F, t33539: F, t33544: F, t33546: F, t4820: F, t5598: F, t787: F) -> (F, F) {
    let t39145 = t39048 * t1;
    let t39146 = t2021 * t39145;
    let t39149 = t2610 * t38912;
    let t39154 = -t33501 - t33505 - F::new(0.79445533226334281486e-1) * t787 * t1858 * t12205 * t2028 + F::new(0.21450293971110256002e1) * t39146 * t2005 + t33508 + t33518 + t33522 - t33526 - t33529 - F::new(0.79445533226334281486e-1) * t5598 * t4820 * t39149 - t33532 + t33536 + t33539 - t33544 - t33546 + F::new(0.10224780254378866581e1) * t28714;
    (t39149, t39154)
}
