//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 676/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk676(t1781: f64, t657: f64, t164: f64, t1774: f64, t25: f64, t5005: f64, t1736: f64, t4953: f64, t4956: f64, t633: f64, t630: f64, t1704: f64, t4907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10871 = t1781 * t1781;
    let t10872 = 1.0_f64 / t10871;
    let t10873 = t657 * t10872;
    let t10879 = t164 * t1774;
    let t10886 = t25 * t5005;
    let t10902 = 1.0_f64 / t4953 / t1736;
    let t10906 = 1.0_f64 / t4956 / t633;
    let t10913 = 1.0_f64 / t4953 / t630;
    let t10924 = 1.0_f64 / t4907 / t1704;
    (t10873, t10879, t10886, t10902, t10906, t10913, t10924)
}
