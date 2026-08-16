//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1046/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1046<F: Float>(t140757: F, t140762: F, t140833: F, t27850: F, t27820: F, t33294: F, t631: F, t97168: F, t1434: F, t150912: F, t193: F, t743: F) -> (F, F, F) {
    let t150953 = t140762 * t140833 * t140757 * t27850;
    let t150958 = t97168 * t631 * t140833 * t33294 * t27820;
    let t150962 = t1434 * t193 * t743 * t150912;
    (t150953, t150958, t150962)
}
