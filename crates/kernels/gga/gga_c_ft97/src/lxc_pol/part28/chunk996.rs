//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 996/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk996<F: Float>(t33196: F, t8392: F, t7350: F, t8232: F, t1882: F, t33180: F, t33163: F, t33092: F, t160: F, t32869: F, t33052: F, t33171: F) -> (F, F, F, F, F, F, F, F) {
    let t140275 = t8392 * t33196;
    let t140278 = F::new(8.0) / F::new(27.0) * t8232 * t7350;
    let t140288 = t1882 * t33180;
    let t140290 = t1882 * t33163;
    let t140325 = t1882 * t33092;
    let t140338 = t160 * t32869;
    let t140364 = t1882 * t33052;
    let t140370 = t1882 * t33171;
    (t140275, t140278, t140288, t140290, t140325, t140338, t140364, t140370)
}
