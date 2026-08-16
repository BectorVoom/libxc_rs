//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 719/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk719<F: Float>(t281: F, t5009: F, t14: F, t816: F, t231: F, t1613: F, t40: F, t7440: F, t7514: F) -> (F, F, F, F, F) {
    let t31535 = t281 * t5009;
    let t31538 = t816 * t14;
    let t31539 = t31538 * t231;
    let t32237 = t40 * t1613;
    let t33243 = t7514 * t7440;
    (t31535, t31538, t31539, t32237, t33243)
}
