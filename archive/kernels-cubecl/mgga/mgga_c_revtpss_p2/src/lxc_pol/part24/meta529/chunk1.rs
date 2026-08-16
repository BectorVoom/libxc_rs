//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1565/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1565<F: Float>(t21251: F, t5373: F, t1219: F, t24551: F, t21254: F, t12772: F, t24797: F, t3625: F, t1256: F, t24684: F, t24700: F, t1803: F, t20850: F) -> (F, F, F, F, F, F, F) {
    let t83994 = t5373 * t21251;
    let t84029 = t24551 * t1219;
    let t84032 = t5373 * t21254;
    let t84061 = t3625 * t12772 * t24797;
    let t84082 = t24684 * t1256;
    let t84084 = t24700 * t1256;
    let t84098 = t20850 * t1803;
    (t83994, t84029, t84032, t84061, t84082, t84084, t84098)
}
