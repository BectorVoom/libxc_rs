//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 806/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk806<F: Float>(t32937: F, t586: F, t28: F, t5890: F, t32892: F, t32896: F, t32902: F, t32910: F, t32915: F, t32919: F, t32923: F, t32927: F, t32931: F, t32935: F) -> (F, F, F) {
    let t32938 = t586 * t32937;
    let t32940 = t5890 * t28 * t32938;
    let t32942 = F::new(3.0) / F::new(2.0) * t32892 + t32896 + F::new(2.0) / F::new(3.0) * t32902 + F::new(4.0) * t32910 - F::new(2.0) * t32915 - t32919 / F::new(2.0) - t32923 - t32927 / F::new(3.0) - F::new(3.0) * t32931 + F::new(2.0) * t32935 + t32940 / F::new(4.0);
    (t32938, t32940, t32942)
}
