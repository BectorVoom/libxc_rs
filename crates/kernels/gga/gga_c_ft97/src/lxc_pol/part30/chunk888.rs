//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 888/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk888<F: Float>(t10157: F, t446: F, t6061: F, t6852: F, t140843: F, t140857: F, t140863: F, t141201: F, t150204: F, t150208: F, t150212: F, t150216: F, t150220: F, t150224: F, t150227: F, t150231: F, t150236: F, t150241: F, t150246: F) -> (F, F) {
    let t150250 = t446 * t10157 * t6852 * t6061;
    let t150253 = -2.0 / 3.0 * t150204 + 2.0 / 9.0 * t150208 - 6.0 * t150212 - 3.0 * t150216 - t150220 / 2.0 + 2.0 * t150224 - t150227 - t150231 / 2.0 + 2.0 * t150236 - 3.0 * t150241 - t140843 / 18.0 + 24.0 * t150246 - 12.0 * t150250 - 2.0 / 3.0 * t140857 + t140863 + t141201;
    (t150250, t150253)
}
