//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 921/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk921<F: Float>(t7: F, t7889: F, t7918: F, t8176: F, t8219: F, t1291: F, t1815: F, t136: F, t154: F, t3188: F, t157: F, t160: F, t163: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t8222 = piecewise3::<F>(t9, F::new(0.0), t7889 + t7918 + t8176 + t8219);
    let t8223 = t1815 * t1291;
    let t8224 = t136 * t8223;
    let t8226 = t154 * t3188;
    let t8231 = t157 * t3188;
    let t8236 = t160 * t3188;
    let t8241 = t163 * t3188;
    (t8222, t8223, t8224, t8226, t8231, t8236, t8241)
}
