//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 733/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk733<F: Float>(t1985: F, t7799: F, t606: F, t7610: F, t1994: F, t599: F, t839: F, t142: F, t2030: F, t1131: F, t604: F, t2060: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7800 = t7799 * t1985;
    let t7802 = t7610 * t606;
    let t7805 = t7799 * t1994;
    let t7807 = t599 * t839;
    let t7808 = t142 * t7807;
    let t7809 = t2030 * t7808;
    let t7811 = t604 * t1131;
    let t7812 = t142 * t7811;
    let t7813 = t2060 * t7812;
    (t7800, t7802, t7805, t7807, t7808, t7809, t7811, t7812, t7813)
}
