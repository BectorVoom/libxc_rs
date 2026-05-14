//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1208/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1208<F: Float>(t18491: F, t24412: F, t13927: F, t27924: F, t6175: F, t68528: F, t4917: F, t747: F, t24437: F, t24519: F, t27762: F, t31036: F, t684: F, t6118: F, t97078: F, t121889: F, t24432: F) -> (F, F, F, F, F, F, F, F) {
    let t122658 = t24412 * t18491;
    let t122662 = t13927 * t27924;
    let t122667 = t68528 * t6175;
    let t122679 = t4917 * t747;
    let t122682 = t24437 * t27762 * t24519 * t122679;
    let t122684 = t31036 * t684;
    let t122686 = t6118 * t97078 * t122684;
    let t122689 = t6118 * t24432 * t121889;
    (t122658, t122662, t122667, t122679, t122682, t122684, t122686, t122689)
}
