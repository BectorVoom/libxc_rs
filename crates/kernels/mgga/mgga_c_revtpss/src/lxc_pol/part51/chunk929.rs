//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 929/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk929<F: Float>(t1043: F, t1089: F, t31935: F, t1976: F, t1982: F, t7150: F, t8507: F, t1035: F, t365: F, t8515: F) -> (F, F, F, F, F) {
    let t31937 = t31935 * t1043 * t1089;
    let t31940 = t1982 * t1976;
    let t31943 = t7150 * t8507;
    let t31948 = t1035 * t365;
    let t31949 = t8515 * t31948;
    (t31937, t31940, t31943, t31948, t31949)
}
