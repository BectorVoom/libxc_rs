//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 773/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk773<F: Float>(t2648: F, t6966: F, t164: F, t2639: F, t1041: F, t5296: F, t177: F, t5305: F, t1037: F, t5384: F, t1769: F, t2663: F) -> (F, F, F, F, F, F) {
    let t6968 = F::new(0.20007875121765877254e-2) * t6966 * t2648;
    let t6970 = t2639 * t164;
    let t6988 = t5296 * t1041;
    let t6990 = t5305 * t177;
    let t6995 = t5384 * t1037;
    let t6998 = F::new(0.40015750243531754508e-1) * t1769 * t2663;
    (t6968, t6970, t6988, t6990, t6995, t6998)
}
