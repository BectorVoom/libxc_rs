//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 977/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk977<F: Float>(t10541: F, t786: F, t2801: F, t231: F, t2645: F, t268: F, t675: F, t2798: F, t760: F, t9323: F, t9318: F, t2251: F, t750: F, t2611: F, t2398: F, t2615: F) -> (F, F, F, F, F, F, F) {
    let t10542 = t786 * t10541;
    let t10543 = t10542 * t2801;
    let t10547 = t268 * t675 * t2645 * t231;
    let t10548 = t2798 * t10547;
    let t10552 = 0.51947577317044391277e2 * t760 * t9323;
    let t10554 = 0.35089341735807877242e1 * t760 * t9318;
    let t10555 = t750 * t2251;
    let t10556 = t2611 * t10555;
    let t10561 = t2398 * t2615;
    (t10542, t10543, t10548, t10552, t10554, t10556, t10561)
}
