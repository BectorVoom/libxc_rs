//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 717/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk717<F: Float>(t160: F, t2086: F, t9641: F, t130: F, t2029: F, t1245: F, t2042: F, t2045: F, t2048: F, t1983: F, t3305: F, t3399: F, t539: F, t544: F, t1264: F, t658: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9677 = t160 * t2086;
    let t9678 = t9641 * t9677;
    let t9686 = t130 * t2029;
    let t9701 = t2042 * t1245;
    let t9703 = t2045 * t1245;
    let t9705 = t2048 * t1245;
    let t9707 = t3305 * t1983;
    let t9715 = t539 * t3399;
    let t9721 = t544 * t3399;
    let t9735 = t1264 * t658;
    (t9678, t9686, t9701, t9703, t9705, t9707, t9715, t9721, t9735)
}
