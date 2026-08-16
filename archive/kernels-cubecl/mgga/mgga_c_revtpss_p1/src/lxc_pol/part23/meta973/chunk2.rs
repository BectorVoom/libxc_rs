//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3300/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3300<F: Float>(t2782: F, t4003: F, t5744: F, t86470: F, t22912: F, t4101: F, t686: F, t72: F, t543: F, t85659: F, t4100: F, t86445: F) -> (F, F, F, F) {
    let t86634 = t2782 * t5744 * t86470 * t4003;
    let t86639 = t4101 * t22912 * t72 * t686;
    let t86641 = t85659 * t543;
    let t86643 = t2782 * t4100 * t86641;
    let t86647 = t2782 * t5744 * t86445 * t4003;
    (t86634, t86639, t86643, t86647)
}
