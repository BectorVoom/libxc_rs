//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1029/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1029<F: Float>(t5989: F, t92978: F, t25277: F, t5985: F, t18352: F, t1945: F, t807: F, t105944: F, t1955: F, t105945: F, t7063: F, t30105: F, t689: F, t1032: F, t6888: F, t1426: F) -> (F, F, F, F, F, F, F, F) {
    let t106082 = t92978 * t5989;
    let t106090 = t25277 * t5985;
    let t106102 = t807 * t1945 * t18352;
    let t106275 = t1955 * t105944;
    let t106387 = t7063 * t105945;
    let t108138 = t30105 * t689;
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    (t106082, t106090, t106102, t106275, t106387, t108138, t108277, t108278)
}
