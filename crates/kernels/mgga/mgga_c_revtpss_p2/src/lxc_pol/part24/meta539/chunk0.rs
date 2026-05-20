//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1585/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1585<F: Float>(t1892: F, t6861: F, t6843: F, t1385: F, t22964: F, t5741: F, t75251: F, t2782: F, t4086: F, t543: F, t86455: F, t14192: F, t86445: F, t9994: F) -> (F, F, F, F, F, F, F) {
    let t86470 = t1892 * t6861;
    let t86506 = t1892 * t6843;
    let t86552 = t1385 * t22964;
    let t86563 = t75251 * t5741;
    let t86575 = t2782 * t4086 * t86455 * t543;
    let t86582 = t2782 * t4086 * t86470 * t543;
    let t86586 = t2782 * t14192 * t86445 * t9994;
    (t86470, t86506, t86552, t86563, t86575, t86582, t86586)
}
