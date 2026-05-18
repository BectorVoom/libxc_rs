//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 935/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk935<F: Float>(t3645: F, t611: F, t103: F, t2162: F, t104: F, t9081: F, t694: F, t9090: F, t9083: F, t96: F, t1662: F, t1679: F, t2541: F) -> (F, F, F, F, F, F) {
    let t32222 = F::new(0.65854491829355115987e0) * t3645 * t611;
    let t32241 = t103 * t2162;
    let t33352 = t104 * t9081;
    let t33357 = F::new(6.0) * t694 * t9090;
    let t33388 = F::new(2.0) * t96 * t9083;
    let t33403 = F::new(2.0) * t1679 * t2541 * t1662;
    (t32222, t32241, t33352, t33357, t33388, t33403)
}
