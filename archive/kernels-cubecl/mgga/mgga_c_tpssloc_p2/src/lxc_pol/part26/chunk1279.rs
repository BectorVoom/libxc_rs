//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1279/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1279<F: Float>(t2690: F, t6619: F, t812: F, t849: F, t6620: F, t9612: F, t23132: F, t2617: F, t23133: F, t2707: F, t6621: F, t9997: F) -> (F, F, F, F, F) {
    let t81763 = t812 * t6619 * t2690;
    let t81764 = t81763 * t849;
    let t81766 = t9612 * t6620;
    let t81767 = t81766 * t849;
    let t81769 = t2617 * t23132;
    let t81770 = t81769 * t849;
    let t81772 = t23133 * t2707;
    let t81774 = t6621 * t9997;
    (t81764, t81767, t81770, t81772, t81774)
}
