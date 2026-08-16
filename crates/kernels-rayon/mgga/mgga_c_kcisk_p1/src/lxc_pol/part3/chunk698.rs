//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 698/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk698(t10880: f64, t1773: f64, t4998: f64, t5025: f64, t25: f64, t5005: f64, t5008: f64, t1744: f64, t4928: f64, t1746: f64, t4948: f64, t4954: f64, t7181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10881 = t1773 * t10880;
    let t10883 = t4998 * t5025;
    let t10884 = t1773 * t10883;
    let t10886 = t25 * t5005;
    let t10887 = t10886 * t5008;
    let t10888 = t1773 * t10887;
    let t10892 = t4928 * t1744;
    let t10893 = t1746 * t4948;
    let t10894 = t10892 * t10893;
    let t10898 = t4954 * t4948 * t7181;
    (t10881, t10884, t10886, t10888, t10894, t10898)
}
