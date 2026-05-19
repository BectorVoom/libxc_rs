//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1115/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1115<F: Float>(t140: F, t147273: F, t147319: F, t147357: F, t147402: F, t147448: F, t147492: F, t147541: F, t147586: F, t138537: F, t6584: F, t32748: F, t6580: F) -> (F, F, F) {
    let t141 = F::new(0.1e-59) < t140;
    let t147590 = piecewise3::<F>(t141, t147273 + t147319 + t147357 + t147402 + t147448 + t147492 + t147541 + t147586, F::new(0.0));
    let t147602 = t138537 * t6584;
    let t147604 = t6580 * t32748;
    (t147590, t147602, t147604)
}
