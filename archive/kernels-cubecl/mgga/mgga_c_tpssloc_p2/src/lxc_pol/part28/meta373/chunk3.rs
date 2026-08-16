//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1421/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1421<F: Float>(t14630: F, t1629: F, t14526: F, t383: F, t1022: F, t4657: F, t1060: F, t14626: F, t3188: F, t1057: F, t14205: F, t11054: F) -> (F, F, F, F, F, F) {
    let t14631 = t1629 * t14630;
    let t14640 = t383 * t14526;
    let t14644 = t4657 * t1022;
    let t14645 = t14644 * t1060;
    let t14648 = t14626 * t3188;
    let t14651 = t14205 * t1057;
    let t14654 = t1629 * t11054;
    (t14631, t14640, t14645, t14648, t14651, t14654)
}
