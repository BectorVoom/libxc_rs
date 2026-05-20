//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2219/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2219<F: Float>(t1066: F, t23485: F, t247: F, t1651: F, t5819: F, t4801: F, t1042: F, t1668: F, t6305: F) -> (F, F, F, F, F) {
    let t23630 = t247 * t1066 * t23485;
    let t23633 = t5819 * t1651;
    let t23634 = t4801 * t23633;
    let t23635 = t1042 * t23634;
    let t23640 = t6305 * t1668;
    (t23630, t23633, t23634, t23635, t23640)
}
