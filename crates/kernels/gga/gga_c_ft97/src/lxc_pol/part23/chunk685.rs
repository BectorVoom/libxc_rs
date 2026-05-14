//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 685/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk685<F: Float>(t18048: F, t18083: F, t202: F, t4985: F, t237: F, t458: F, t4966: F, t17749: F, t9568: F, t92: F, t17766: F, t2404: F, t17753: F, t3051: F, t4970: F, t17744: F, t683: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18084 = t18048 + t18083;
    let t18089 = t202 * t4985;
    let t18090 = t18089 * t237;
    let t18096 = t458 * t4966;
    let t18098 = t9568 * t17749;
    let t18099 = t92 * t18098;
    let t18101 = t2404 * t17766;
    let t18102 = t92 * t18101;
    let t18104 = t2404 * t17753;
    let t18105 = t3051 * t18104;
    let t18107 = t458 * t4970;
    let t18109 = t683 * t17744;
    (t18084, t18089, t18090, t18096, t18099, t18102, t18105, t18107, t18109)
}
