//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1126/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1126<F: Float>(t27279: F, t7058: F, t72: F, t7769: F, t686: F, t25375: F, t25387: F, t1955: F, t7057: F, t1949: F, t2718: F, t2411: F, t7782: F) -> (F, F, F, F, F, F, F, F) {
    let t27338 = t7058 * t27279;
    let t27340 = t7769 * t72;
    let t27341 = t27340 * t686;
    let t27342 = t25375 * t27341;
    let t27344 = t25387 * t27341;
    let t27353 = t1955 * t7057;
    let t27357 = t2718 * t1949;
    let t27368 = t7782 * t2411;
    (t27338, t27340, t27341, t27342, t27344, t27353, t27357, t27368)
}
