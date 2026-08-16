//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1237/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1237<F: Float>(t2075: F, t7467: F, t652: F, t1458: F, t8595: F, t2095: F, t33136: F, t1983: F, t1873: F, t27254: F, t24465: F, t7769: F) -> (F, F, F, F, F, F, F, F) {
    let t33617 = t2075 * t7467;
    let t33619 = F::cast_from(2.0_f64) * t652 * t33617;
    let t33620 = t8595 * t1458;
    let t33622 = F::cast_from(2.0_f64) * t652 * t33620;
    let t33623 = t2095 * t33136;
    let t33624 = t1983 * t33623;
    let t33641 = F::cast_from(0.135e2_f64) * t27254 * t1873;
    let t33643 = F::cast_from(27.0_f64) * t24465 * t7769;
    (t33617, t33619, t33620, t33622, t33623, t33624, t33641, t33643)
}
