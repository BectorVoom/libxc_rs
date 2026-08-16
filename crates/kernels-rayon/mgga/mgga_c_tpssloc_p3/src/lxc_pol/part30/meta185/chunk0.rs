//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 904/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk904(t3451: f64, t4919: f64, t3295: f64, t3464: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64, t457: f64, t460: f64, t974: f64, t1184: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4920 = t4919 * t3451;
    let t4928 = -t3464 + t3295 / 9.0_f64 + t4770 / 9.0_f64 + t4773 / 18.0_f64 - t4776 / 3.0_f64 - t4779 / 6.0_f64;
    let t4929 = t457 * t4928;
    let t4930 = t4929 * t460;
    let t4931 = t974 * t4930;
    let t4934 = t974 * t457;
    let t4935 = t1714 * t1184;
    (t4920, t4928, t4930, t4931, t4934, t4935)
}
