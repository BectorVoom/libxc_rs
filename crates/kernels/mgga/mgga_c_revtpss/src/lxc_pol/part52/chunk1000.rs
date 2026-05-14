//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1000/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1000<F: Float>(t84: F, t8440: F, t25081: F, t8567: F, t31844: F, t8478: F, t8479: F, t246: F, t826: F, t854: F, t2718: F, t843: F, t8484: F, t839: F, t31752: F, t31753: F) -> (F, F, F, F, F, F, F, F) {
    let t119457 = t8440 * t84;
    let t119578 = t8567 * t25081;
    let t119751 = t8478 * t8479 * t31844;
    let t119752 = t826 * t246;
    let t119757 = t854 * t246;
    let t119763 = t8478 * t8484 * t2718 * t843;
    let t119764 = t119763 * t839;
    let t119765 = 0.34708173928447610098e-2 * t119764;
    let t119767 = t31752 * t31753 * t854;
    (t119457, t119578, t119751, t119752, t119757, t119763, t119765, t119767)
}
