//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 360/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk360<F: Float>(t222: F, t1905: F, t1958: F, t2038: F, t2042: F, t2049: F, t240: F, t802: F, t1055: F, t143: F, t224: F, zeta_threshold: F) -> (F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t2053 = t1905 - t1958 + t240 * (t2038 * t802 - t2042 * t2049 - t1905 + t1958);
    let t2059 = -t143 - t1055;
    let t2062 = piecewise3(t223, 0.0, 4.0 / 3.0 * t224 * t2059);
    let t2063 = -t2059;
    (t2053, t2059, t2062, t2063)
}
