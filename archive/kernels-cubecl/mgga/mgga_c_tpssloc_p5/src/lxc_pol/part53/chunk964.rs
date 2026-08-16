//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 964/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk964<F: Float>(t4234: F, t6605: F, t6612: F, t25119: F, t4255: F, t6619: F, t23046: F, t25093: F, t23097: F, t25097: F, t112792: F, t4184: F) -> (F, F, F, F, F) {
    let t118549 = t6605 * t6612 * t4234;
    let t118552 = t25119 * t6619 * t4255;
    let t118556 = t6605 * t23046 * t25093;
    let t118559 = t23097 * t6612 * t25097;
    let t118562 = t112792 * t4184;
    (t118549, t118552, t118556, t118559, t118562)
}
