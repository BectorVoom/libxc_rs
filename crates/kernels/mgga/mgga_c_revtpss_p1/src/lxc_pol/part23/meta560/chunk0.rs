//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2125/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2125<F: Float>(t1843: F, t5920: F, t1513: F, t5891: F, t10208: F, t4263: F, t5915: F, t1504: F, t5895: F, t10227: F, t4269: F, t5823: F) -> (F, F, F, F, F, F, F) {
    let t22578 = t1843 * t5920;
    let t22589 = t5891 * t1513;
    let t22590 = t10208 * t22589;
    let t22593 = t4263 * t5915;
    let t22596 = t5895 * t1504;
    let t22597 = t10227 * t22596;
    let t22600 = t4269 * t5823;
    (t22578, t22589, t22590, t22593, t22596, t22597, t22600)
}
