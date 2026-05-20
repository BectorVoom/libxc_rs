//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3089/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3089<F: Float>(t12621: F, t1774: F, t1214: F, t16750: F, t12629: F, t3555: F, t5412: F, t1269: F, t5216: F, t3565: F, t5215: F, t487: F) -> (F, F, F, F, F, F, F) {
    let t56543 = t1774 * t12621;
    let t56555 = t16750 * t1214;
    let t56561 = t1774 * t12629;
    let t56570 = t3555 * t5412;
    let t56575 = t5216 * t1269;
    let t56587 = t5215 * t3565;
    let t56588 = t56587 * t487;
    (t56543, t56555, t56561, t56570, t56575, t56587, t56588)
}
