//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1026/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1026<F: Float>(t103431: F, t25375: F, t103421: F, t7058: F, t11064: F, t8019: F, t5891: F, t94978: F, t25823: F, t5915: F, t29682: F, t689: F, t1032: F, t6041: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
    let t103521 = t25375 * t103431;
    let t103547 = t7058 * t103421;
    let t103586 = t8019 * t11064;
    let t105870 = t94978 * t5891;
    let t105878 = t25823 * t5915;
    let t105936 = t29682 * t689;
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    (t103521, t103547, t103586, t105870, t105878, t105936, t105944, t105945, t105946)
}
