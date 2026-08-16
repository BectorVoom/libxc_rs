//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1122/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1122(t18929: f64, t26: f64, t18672: f64, t2970: f64, t6383: f64, t659: f64, t6386: f64, t13710: f64, t13945: f64, t13949: f64, t18924: f64, t18927: f64, t9726: f64, t9729: f64) -> (f64, f64, f64, f64, f64) {
    let t18930 = t26 * t18929;
    let t18932 = t2970 * t18672;
    let t18933 = t26 * t18932;
    let t18935 = t659 * t6383;
    let t18937 = t659 * t6386;
    let t18942 = -0.49293999999999999999e0_f64 * t18924 + 0.65725333333333333332e0_f64 * t18927 + 0.16431333333333333333e0_f64 * t18930 - 0.27385555555555555556e-1_f64 * t18933 - t9726 - t9729 - 0.10954222222222222222e0_f64 * t18935 + 0.54771111111111111111e-1_f64 * t18937 - 0.18257037037037037037e0_f64 * t13945 - 0.26574814814814814815e0_f64 * t13710 + 0.21908444444444444444e0_f64 * t13949;
    (t18930, t18933, t18935, t18937, t18942)
}
