//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 378/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk378<F: Float>(t2018: F, t213: F, t552: F, t1955: F, t555: F, t1032: F, t1426: F) -> (F, F, F, F) {
    let t2019 = t213 * t2018;
    let t2020 = t2019 * t552;
    let t2027 = t1955 * t555;
    let t2028 = t1032 * t1426;
    (t2019, t2020, t2027, t2028)
}
