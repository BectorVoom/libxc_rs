//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 864/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk864<F: Float>(t34512: F, t83: F, t34737: F, t34742: F, t34746: F, t34750: F, t34754: F, t34758: F, t34762: F, t34765: F, t34770: F, t34773: F, t34776: F, t446: F) -> (F, F) {
    let t34779 = t83 * t34512;
    let t34782 = F::new(2.0) / F::new(3.0) * t446 * t34737 - F::new(2.0) / F::new(3.0) * t446 * t34742 + F::new(4.0) / F::new(3.0) * t446 * t34746 - F::new(2.0) * t446 * t34750 - t446 * t34754 / F::new(3.0) - t446 * t34758 / F::new(3.0) - t446 * t34762 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t34765 + t446 * t34770 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t34773 - F::new(2.0) / F::new(3.0) * t446 * t34776 - t446 * t34779 / F::new(3.0);
    (t34779, t34782)
}
