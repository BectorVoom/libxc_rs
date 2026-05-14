//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1117/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1117<F: Float>(t5063: F, t7316: F, t33097: F, t5291: F, t9704: F, t4797: F, t4972: F, t7303: F, t9708: F, t4803: F, t5290: F, t10375: F, t748: F, t1950: F, t4817: F, t1954: F, t4581: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33098 = t7316 * t5063;
    let t33099 = t33097 * t33098;
    let t33101 = t9704 * t5291;
    let t33103 = t7316 * t4797;
    let t33104 = t9704 * t33103;
    let t33106 = t7303 * t4972;
    let t33107 = t9708 * t33106;
    let t33109 = t5290 * t4803;
    let t33110 = t9708 * t33109;
    let t33112 = t10375 * t748;
    let t33114 = t4817 * t1950;
    let t33116 = t4581 * t1954;
    (t33098, t33099, t33101, t33103, t33104, t33106, t33107, t33109, t33110, t33112, t33114, t33116)
}
