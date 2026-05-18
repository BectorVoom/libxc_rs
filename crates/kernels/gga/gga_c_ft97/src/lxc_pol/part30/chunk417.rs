//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 417/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk417<F: Float>(t27: F, t6903: F, t89: F, t6117: F, t6134: F, t6881: F, t6885: F, t6889: F, t6893: F, t6897: F, t6901: F) -> (F, F) {
    let t6905 = t89 * t27 * t6903;
    let t6907 = t6881 / F::new(12.0) + t6117 + t6885 / F::new(18.0) + t6889 / F::new(3.0) - t6893 / F::new(6.0) + t6134 + t6897 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t6901 - t6905 / F::new(3.0);
    (t6905, t6907)
}
