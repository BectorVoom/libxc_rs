//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 824/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk824<F: Float>(t7049: F, t7266: F, t589: F, t2069: F, t5897: F, t4189: F, t4301: F, t6922: F, t583: F, t578: F, t2035: F, t2043: F, t2047: F, t556: F, t7257: F, t572: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7267 = t7049 + t7266;
    let t7268 = t7267 * t589;
    let t7270 = 2.0 * t5897 * t2069;
    let t7271 = t2069 * t2069;
    let t7273 = 2.0 * t4189 * t7271;
    let t7274 = t4301 * t6922;
    let t7275 = t583 * t7274;
    let t7276 = t578 * t7275;
    let t7278 = t2035 * t2043;
    let t7280 = t2035 * t2047;
    let t7282 = t556 * t7257;
    let t7283 = t572 * t7282;
    (t7267, t7268, t7270, t7271, t7273, t7275, t7276, t7278, t7280, t7282, t7283)
}
