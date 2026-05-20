//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2497/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2497<F: Float>(t10199: F, t1514: F, t2289: F, t4264: F, t10227: F, t97: F, t10241: F, t105: F, t4288: F, t4398: F, t9372: F, t1469: F, t2608: F, t4401: F, t606: F) -> (F, F, F, F, F, F, F) {
    let t49698 = t10199 * t1514;
    let t49700 = t2289 * t4264;
    let t49701 = F::new(22.0) / F::new(3.0) * t49700;
    let t49777 = t97 * t10227;
    let t49787 = t105 * t10241;
    let t49817 = t2289 * t4288;
    let t49818 = F::new(11.0) / F::new(3.0) * t49817;
    let t49866 = t4398 * t9372;
    let t49876 = t4401 * t2608 * t1469 * t606;
    (t49698, t49701, t49777, t49787, t49818, t49866, t49876)
}
