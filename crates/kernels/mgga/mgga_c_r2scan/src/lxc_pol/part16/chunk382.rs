//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 382/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk382<F: Float>(t1604: F, t1607: F, t774: F, t784: F, t783: F, t788: F, t162: F, t38: F) -> (F, F, F, F) {
    let t1608 = t1604 * t1607;
    let t1610 = t774 * t784;
    let t1612 = t783 * t1610 * t788;
    let t1614 = t162 * t38;
    let t1615 = F::new(1.0) / t1614;
    (t1608, t1610, t1612, t1615)
}
