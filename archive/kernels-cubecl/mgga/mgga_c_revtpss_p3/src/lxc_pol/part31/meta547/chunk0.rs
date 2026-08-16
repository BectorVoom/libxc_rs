//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1939/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1939<F: Float>(t29731: F, t7160: F, t1668: F, t7817: F, t1089: F, t7821: F, t1646: F, t7810: F, t7145: F, t1976: F, t6350: F) -> (F, F, F, F, F, F, F, F) {
    let t29732 = t7160 * t29731;
    let t29739 = t7817 * t1668;
    let t29740 = t29739 * t1089;
    let t29743 = t7821 * t1668;
    let t29744 = t29743 * t1089;
    let t29747 = t7810 * t1646;
    let t29748 = t7145 * t29747;
    let t29751 = t1976 * t6350;
    (t29732, t29739, t29740, t29743, t29744, t29747, t29748, t29751)
}
