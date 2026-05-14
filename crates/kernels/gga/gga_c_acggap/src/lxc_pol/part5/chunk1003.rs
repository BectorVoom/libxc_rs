//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1003/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1003<F: Float>(t11731: F, t11733: F, t14941: F, t11748: F, t11750: F, t11762: F, t229: F, t6012: F, t224: F, t6008: F, t14957: F, t14959: F, t14965: F, t11739: F, t11743: F, t11747: F, t11756: F, t11770: F, t11772: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19977 = 0.5848223622634646207e0 * t11731;
    let t19978 = 0.17315859105681463759e2 * t11733;
    let t19979 = 80.0 * t14941;
    let t19980 = 8.0 * t11748;
    let t19981 = 20.0 * t11750;
    let t19982 = 32.0 * t11762;
    let t19983 = t229 * t6012;
    let t19984 = 8.0 * t19983;
    let t19985 = t224 * t6012;
    let t19986 = 8.0 * t19985;
    let t19987 = t229 * t6008;
    let t19988 = 8.0 * t19987;
    let t19989 = 24.0 * t14957;
    let t19990 = 24.0 * t14959;
    let t19991 = 48.0 * t14965;
    let t19992 = -t19977 - t19978 + t11739 - t11743 + t19979 + t11747 - t19980 + t19981 - t11756 + t19982 - t19984 + t19986 - t19988 - t19989 - t19990 - t19991 + t11770 - t11772;
    (t19977, t19978, t19979, t19980, t19981, t19982, t19984, t19986, t19988, t19989, t19990, t19991, t19992)
}
