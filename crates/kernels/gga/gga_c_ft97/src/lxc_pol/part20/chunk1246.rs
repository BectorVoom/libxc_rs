//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1246/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1246<F: Float>(t113001: F, t1486: F, t193: F, t2781: F, t1476: F, t14889: F, t112671: F, t43381: F, t446: F, t113511: F, t113516: F, t113519: F, t113522: F, t113527: F, t99535: F, t99537: F, t99806: F) -> (F, F, F, F, F) {
    let t113530 = t1486 * t193 * t2781 * t113001;
    let t113531 = t1476 * t14889;
    let t113534 = t1486 * t193 * t2781 * t113531;
    let t113536 = t446 * t43381 * t112671;
    let t113539 = t113511 / 3.0 - t113516 / 9.0 - 4.0 / 3.0 * t113519 - 2.0 / 3.0 * t113522 + t113527 + t113530 + t113534 + 2.0 * t113536 + t99535 - 8.0 / 9.0 * t99537 + t99806;
    (t113530, t113531, t113534, t113536, t113539)
}
