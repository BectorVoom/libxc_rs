//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1074/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1074(t37890: f64, t531: f64, t10743: f64, t2233: f64, t10740: f64, t776: f64, t2080: f64, t3344: f64, t1050: f64, t120: f64, t20621: f64, t10698: f64, t10701: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37891 = t37890 * t531;
    let t37893 = t10743 * t2233;
    let t37903 = t776 * t10740;
    let t37905 = t2080 * t3344;
    let t37919 = t120 * t20621 * t1050;
    let t37925 = t10698 * t10701;
    (t37891, t37893, t37903, t37905, t37919, t37925)
}
