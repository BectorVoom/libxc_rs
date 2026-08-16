//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 659/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk659<F: Float>(t1391: F, t502: F, t1390: F, t386: F, t518: F, t385: F, t4715: F, t5: F) -> (F, F, F, F, F) {
    let t4735 = t1391 * t502;
    let t4736 = t1390 * t4735;
    let t4738 = t386 * t518;
    let t4739 = t385 * t4738;
    let t4741 = t5 * t4715;
    (t4735, t4736, t4738, t4739, t4741)
}
