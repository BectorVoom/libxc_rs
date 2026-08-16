//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1073/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1073(t120: f64, t6511: f64, t531: f64, t10740: f64, t776: f64, t1050: f64, t20621: f64, t2090: f64, t3294: f64, t3296: f64, t2096: f64, t2167: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37890 = t120 * t6511;
    let t37891 = t37890 * t531;
    let t37903 = t776 * t10740;
    let t37919 = t120 * t20621 * t1050;
    let t37932 = t120 * t2090 * t3294;
    let t37933 = t37932 * t3296;
    let t37935 = t2167 * t2096;
    (t37890, t37891, t37903, t37919, t37932, t37933, t37935)
}
