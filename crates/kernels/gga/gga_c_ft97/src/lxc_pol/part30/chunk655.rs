//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 655/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk655<F: Float>(t668: F, t7021: F, t505: F, t2665: F, t446: F, t28491: F, t28494: F, t28499: F, t28504: F, t28509: F, t28514: F, t28518: F, t28522: F, t28526: F, t28529: F, t28531: F) -> (F, F, F) {
    let t28533 = t7021 * t668;
    let t28534 = t28533 * t505;
    let t28535 = t2665 * t28534;
    let t28536 = t446 * t28535;
    let t28538 = t28491 / F::new(9.0) - t28494 / F::new(36.0) + t28499 / F::new(3.0) + t28504 / F::new(3.0) + t28509 / F::new(3.0) + t28514 / F::new(12.0) - F::new(2.0) / F::new(9.0) * t28518 - F::new(2.0) / F::new(9.0) * t28522 + F::new(2.0) / F::new(27.0) * t28526 + t28529 / F::new(18.0) - t28531 / F::new(27.0) + t28536 / F::new(9.0);
    (t28534, t28536, t28538)
}
