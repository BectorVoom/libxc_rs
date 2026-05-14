//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 557/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk557<F: Float>(t1284: F, t8010: F, t487: F, t486: F, t382: F, t7831: F, t467: F, t8161: F, t492: F, t500: F, t2275: F, t6382: F, t2271: F, t2279: F, t499: F, t8072: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8251 = t1284 * t8010;
    let t8252 = t487 * t8251;
    let t8253 = t486 * t8252;
    let t8255 = t382 * t7831;
    let t8256 = t487 * t8255;
    let t8257 = t486 * t8256;
    let t8259 = t8161 * t467;
    let t8260 = t8259 * t492;
    let t8261 = t8260 * t500;
    let t8263 = t6382 * t2275;
    let t8265 = t2271 * t2279;
    let t8267 = t499 * t8072;
    (t8251, t8252, t8253, t8255, t8256, t8257, t8259, t8260, t8261, t8263, t8265, t8267)
}
