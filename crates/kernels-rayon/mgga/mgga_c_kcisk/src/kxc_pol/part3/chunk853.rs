//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 853/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk853(t1187: f64, t2877: f64, t3668: f64, t827: f64, t3662: f64, t3951: f64, t79: f64, t12831: f64, t26: f64, t1186: f64, t12925: f64, t3580: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12935 = t2877 * t1187;
    let t12937 = t827 * t3668;
    let t12939 = t827 * t3662;
    let t12941 = t79 * t3951;
    let t12942 = t12941 * t12831;
    let t12943 = t26 * t12942;
    let t12945 = t1186 * t12925;
    let t12946 = t26 * t12945;
    let t12948 = t821 * t3580;
    (t12935, t12937, t12939, t12943, t12946, t12948)
}
