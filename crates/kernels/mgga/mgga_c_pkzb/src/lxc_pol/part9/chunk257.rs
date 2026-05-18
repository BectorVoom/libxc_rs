//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 257/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk257<F: Float>(t12: F, t318: F, t319: F, t808: F, t810: F, t201: F, t204: F, t334: F, t648: F, t92: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t814 = piecewise3::<f64>(t203, F::new(0.0), t318 * t810 / F::new(2.0) + t808 * t319 / F::new(2.0));
    let t815 = t201 * t814;
    let t819 = t204 * t648 * t334;
    let t820 = F::new(0.17808333333333333333e-1) * t819;
    let t821 = F::new(1.0) / t92;
    (t815, t819, t820, t821)
}
