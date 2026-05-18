//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 853/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk853<F: Float>(t35678: F, t762: F, t242: F, t241: F, t258: F, t35546: F, t1131: F, t729: F, t7560: F, t193: F, t33707: F, t33747: F, t33765: F, t35636: F, t35641: F, t35645: F, t35649: F, t35653: F, t35657: F, t446: F, t89: F) -> (F, F, F, F, F) {
    let t35679 = t762 * t35678;
    let t35680 = t242 * t35679;
    let t35684 = t241 * t35546 * t258;
    let t35689 = t729 * t7560 * t1131;
    let t35692 = -t33707 + t446 * t35636 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t35641 + F::new(2.0) / F::new(3.0) * t446 * t35645 + F::new(4.0) / F::new(3.0) * t446 * t35649 + F::new(4.0) / F::new(3.0) * t446 * t35653 - F::new(2.0) * t446 * t35657 + t33747 - t33765 - t446 * t35680 / F::new(3.0) + t89 * t193 * t35684 / F::new(3.0) - t446 * t35689 / F::new(3.0);
    (t35679, t35680, t35684, t35689, t35692)
}
