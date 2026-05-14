//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1405/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1405<F: Float>(t117327: F, t9977: F, t24140: F, t33097: F, t34313: F, t34324: F, t24254: F, t34368: F, t6974: F, t7437: F, t1954: F, t22254: F, t117409: F, t5061: F, t7431: F, t22952: F, t7316: F, t9704: F) -> (F, F, F, F, F, F, F, F) {
    let t122267 = t117327 * t9977;
    let t122269 = t33097 * t24140;
    let t122271 = t34313 * t34324;
    let t122273 = t34368 * t24254;
    let t122275 = t6974 * t7437;
    let t122277 = t22254 * t1954;
    let t122280 = t5061 * t117409 * t7431;
    let t122283 = t9704 * t7316 * t22952;
    (t122267, t122269, t122271, t122273, t122275, t122277, t122280, t122283)
}
