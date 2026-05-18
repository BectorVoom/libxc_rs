//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 302/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk302<F: Float>(t298: F, t990: F, t302: F, t308: F, t295: F, t305: F, t309: F, t814: F, t313: F, t825: F, rho1: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t991 = t298 * t990;
    let t994 = rho1 * rho1;
    let t996 = F::new(1.0) / t302 / t994;
    let t997 = tau1 * t996;
    let t1000 = -t990;
    let t1001 = t308 * t1000;
    let t1004 = F::new(5.0) / F::new(3.0) * t295 * t991 - F::new(5.0) / F::new(3.0) * t997 * t309 + F::new(5.0) / F::new(3.0) * t305 * t1001 + t814;
    let t1010 = F::new(3.0) / F::new(10.0) * t313 * (F::new(5.0) / F::new(3.0) * t991 + F::new(5.0) / F::new(3.0) * t1001) - t825;
    (t991, t994, t997, t1000, t1001, t1004, t1010)
}
