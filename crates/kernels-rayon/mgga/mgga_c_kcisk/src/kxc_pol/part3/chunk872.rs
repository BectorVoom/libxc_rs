//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 872/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk872(t13083: f64, t13098: f64, t1203: f64, t1212: f64, t12885: f64, t3722: f64, t12974: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12948: f64, t12954: f64, t12959: f64, t12985: f64, t12989: f64) -> (f64, f64, f64, f64) {
    let t13099 = t13083 + t13098;
    let t13101 = t1203 * t13099 * t1212;
    let t13105 = t3722 * t12885 * t1212;
    let t13110 = 0.55403703703703703703e-1_f64 * t12974;
    let t13121 = -t13110 - 0.23744444444444444444e-1_f64 * t12929 + 0.11872222222222222222e-1_f64 * t12933 - 0.35616666666666666666e-1_f64 * t12948 + 0.17808333333333333333e-1_f64 * t12931 - 0.19787037037037037037e-1_f64 * t12922 + 0.71233333333333333332e-1_f64 * t12954 - 0.35616666666666666666e-1_f64 * t12985 - 0.10685e0_f64 * t12959 + 0.10685e0_f64 * t12989 - 0.17808333333333333333e-1_f64 * t12927;
    (t13099, t13101, t13105, t13121)
}
