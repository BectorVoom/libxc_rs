//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 812/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk812<F: Float>(t225: F, t6005: F, t2638: F, t5966: F, t5962: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t231: F) -> (F, F, F, F, F) {
    let t6006 = t6005 * t225;
    let t6010 = t2638 * t5966;
    let t6013 = t832 * t5962;
    let t6016 = 6.0 * t1553 * t1555 - 12.0 * t227 * t6010 + 3.0 * t227 * t6013 - t229 * t6006;
    let t6017 = t6016 * t231;
    (t6006, t6010, t6013, t6016, t6017)
}
