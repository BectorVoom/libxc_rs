//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 995/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk995<F: Float>(t33587: F, t6745: F, t24429: F, t6940: F, t140774: F, t3886: F, t2917: F, t33292: F, t7515: F, t35353: F, t684: F, t24432: F, t6118: F) -> (F, F, F, F, F, F) {
    let t150023 = t6745 * t33587;
    let t150031 = t24429 * t6940;
    let t150034 = t140774 * t3886;
    let t150036 = t33292 * t2917 * t7515 * t150034;
    let t150038 = t35353 * t684;
    let t150040 = t6118 * t24432 * t150038;
    (t150023, t150031, t150034, t150036, t150038, t150040)
}
