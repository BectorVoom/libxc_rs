//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1208/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1208<F: Float>(t12712: F, t471: F, t6688: F, t3720: F, t1774: F, t3367: F, t4181: F, t3626: F, t6622: F, t73: F, t5352: F, t20956: F, t5333: F, t17934: F, t5330: F, t5327: F, t5362: F) -> (F, F, F, F, F, F, F) {
    let t21028 = t12712 * t471;
    let t21029 = t6688 * t21028;
    let t21030 = t3720 * t21029;
    let t21035 = t1774 * t3367;
    let t21036 = t21035 * t4181;
    let t21037 = t3626 * t21036;
    let t21040 = t6622 * t73;
    let t21041 = t21040 * t5352;
    let t21042 = t3720 * t21041;
    let t21045 = t20956 * t5333;
    let t21046 = t3720 * t21045;
    let t21049 = t17934 * t5330;
    let t21053 = t5327 * t5362;
    (t21030, t21037, t21040, t21042, t21046, t21049, t21053)
}
