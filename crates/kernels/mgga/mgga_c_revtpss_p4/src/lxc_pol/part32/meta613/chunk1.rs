//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1953/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1953<F: Float>(t1398: F, t543: F, t6918: F, t1955: F, t27883: F, t1444: F, t6844: F, t1903: F, t5658: F, t1032: F, t6888: F, t1426: F) -> (F, F, F, F, F, F) {
    let t108206 = t6918 * t1398 * t543;
    let t108225 = t1955 * t27883;
    let t108244 = t6844 * t1444;
    let t108259 = t1903 * t5658 * t543;
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    (t108206, t108225, t108244, t108259, t108277, t108278)
}
