//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 858/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk858<F: Float>(t8117: F, t8174: F, t8220: F, t8256: F, t8303: F, t8343: F, t8392: F, t8432: F, t2967: F, t747: F, t1052: F, t1961: F) -> (F, F, F) {
    let t8435 = t8117 + t8174 + t8220 + t8256 + t8303 + t8343 + t8392 + t8432;
    let t8440 = t2967 * t747;
    let t8443 = t1052 * t1961;
    (t8435, t8440, t8443)
}
