//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 923/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk923<F: Float>(t1621: F, t19275: F, t1634: F, t6205: F, t1633: F, t19303: F, t1610: F, t6141: F, t2874: F, t1609: F, t19330: F, t2924: F, t1622: F, t6173: F, t11452: F, t23705: F) -> (F, F, F, F, F, F, F) {
    let t23758 = t19275 * t1621;
    let t23761 = t1634 * t6205;
    let t23764 = t19303 * t1633;
    let t23767 = t1610 * t6141;
    let t23769 = 6.0 * t2874 * t23767;
    let t23770 = t19330 * t1609;
    let t23772 = 0.48245938496077605201e2 * t2924 * t23770;
    let t23773 = t1622 * t6173;
    let t23776 = t23705 * t11452;
    (t23758, t23761, t23764, t23769, t23772, t23773, t23776)
}
