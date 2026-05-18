//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 814/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk814<F: Float>(t32992: F, t605: F, t7400: F, t9276: F, t32967: F, t5779: F, t28: F, t1389: F, t558: F, t5778: F, t1882: F, t7363: F) -> (F, F, F, F, F, F, F, F) {
    let t32993 = t605 * t32992;
    let t32995 = t9276 * t7400;
    let t32997 = t32967 * t5779;
    let t32998 = t28 * t32997;
    let t33000 = t1389 * t558;
    let t33001 = t5778 * t33000;
    let t33002 = t28 * t33001;
    let t33008 = t1882 * t7363 / F::new(9.0);
    (t32993, t32995, t32997, t32998, t33000, t33001, t33002, t33008)
}
