//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1312/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1312<F: Float>(t18986: F, t18991: F, t795: F, t797: F, t97: F, t9937: F, t19000: F, t10265: F, t879: F, t2266: F, t9573: F, t9577: F, t18984: F, t18990: F, t18995: F, t23741: F, t23752: F, t32131: F, t372: F) -> (F, F, F, F, F, F) {
    let t32133 = 0.21687162600603479684e-1 * t18986;
    let t32134 = 12.0 * t18991;
    let t32138 = 6.0 * t97 * t9937 * t795 * t797;
    let t32139 = 0.10389515463408878255e3 * t19000;
    let t32140 = t879 * t10265;
    let t32143 = 18.0 * t2266 * t9573 * t9577;
    let t32144 = t32131 * t372 - t18984 + t18990 - t18995 + t23741 - t23752 + t32133 + t32134 - t32138 - t32139 + t32140 - t32143;
    (t32133, t32134, t32138, t32139, t32143, t32144)
}
