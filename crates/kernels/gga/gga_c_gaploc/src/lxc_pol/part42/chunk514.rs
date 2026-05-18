//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 514/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk514<F: Float>(t2685: F, t9829: F, t2684: F, t2465: F, t2581: F, t2464: F, t3311: F, t7416: F, t2013: F, t3296: F, t969: F, t825: F) -> (F, F, F, F, F) {
    let t9830 = t2685 * t9829;
    let t9831 = t2684 * t9830;
    let t9833 = t2465 * t2581;
    let t9834 = t2464 * t9833;
    let t9835 = t2684 * t9834;
    let t9837 = t7416 * t3311;
    let t9845 = t2013 * t3296;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    (t9831, t9835, t9837, t9845, t9848)
}
