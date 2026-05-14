//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 730/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk730<F: Float>(t538: F, t7605: F, t6155: F, t1634: F, t2651: F, t252: F, t5094: F, t146: F) -> (F, F, F, F, F) {
    let t7606 = t538 * t7605;
    let t7608 = 0.10975748638225852664e-1 * t6155 * t7606;
    let t7610 = 0.23115257973478049502e0 * t2651 * t1634;
    let t7613 = t5094 * t252;
    let t7614 = t146 * t7613;
    (t7606, t7608, t7610, t7613, t7614)
}
