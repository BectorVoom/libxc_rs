//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 839/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk839<F: Float>(t12: F, t6069: F, t6070: F, t5100: F, t2159: F, t2163: F, t318: F, t319: F, t808: F, t810: F, t201: F, t199: F, t204: F, t334: F, t3981: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t6071 = t6069 + t6070;
    let t6078 = piecewise3::<f64>(t84, F::new(0.0), t5100);
    let t6082 = piecewise3::<f64>(t203, F::new(0.0), t6071 * t319 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2159 * t810 + F::new(3.0) / F::new(2.0) * t808 * t2163 + t318 * t6078 / F::new(2.0));
    let t6083 = t201 * t6082;
    let t6084 = t199 * t6083;
    let t6085 = F::new(0.2390625e-1) * t6084;
    let t6087 = t204 * t3981 * t334;
    (t6071, t6078, t6085, t6087)
}
