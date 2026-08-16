//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1097/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1097(t154: f64, t18086: f64, t1885: f64, t276: f64, t5645: f64, t735: f64, t2899: f64, t5704: f64, t774: f64, t2922: f64, t5961: f64, t5975: f64, t5984: f64) -> (f64, f64, f64, f64, f64) {
    let t18089 = t276 * t154 * t18086 * t1885;
    let t18091 = t735 * t5645;
    let t18094 = t2899 * t774 * t5704;
    let t18097 = t2922 * t774 * t5961;
    let t18103 = t5984 * t5975;
    (t18089, t18091, t18094, t18097, t18103)
}
