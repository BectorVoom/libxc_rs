//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1520/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1520(t15717: f64, t996: f64, t1678: f64, t3057: f64, t15648: f64, t16152: f64, t15837: f64, t4930: f64, t994: f64, t3046: f64, t1000: f64, t11187: f64, t11201: f64, t11220: f64, t1680: f64, t1696: f64, t3043: f64, t3047: f64, t3058: f64, t3060: f64, t3063: f64, t3264: f64, t3271: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4941: f64, t4947: f64, t995: f64) -> f64 {
    let t16275 = t996 * t15717;
    let t16284 = t3057 * t1678;
    let t16287 = t996 * t15648;
    let t16292 = t996 * t16152;
    let t16295 = t996 * t15837;
    let t16302 = t994 * t4930;
    let t16305 = t3046 * t1678;
    let t16310 = 0.13170898365871023197e1_f64 * t3047 * t4764 - 0.39512695097613069591e1_f64 * t11201 * t16275 + 0.26341796731742046394e1_f64 * t3264 * t4947 - 0.13170898365871023197e1_f64 * t11220 * t1696 + 0.13170898365871023197e1_f64 * t4752 * t3271 + 0.13170898365871023197e1_f64 * t16284 * t3060 - 0.65854491829355115987e0_f64 * t995 * t16287 + 0.26341796731742046394e1_f64 * t11187 * t4758 + 0.26341796731742046394e1_f64 * t3058 * t16292 + 0.13170898365871023197e1_f64 * t3058 * t16295 + 0.65854491829355115987e0_f64 * t3043 * t1680 - 0.13170898365871023197e1_f64 * t3063 * t4773 - 0.13170898365871023197e1_f64 * t16302 * t1000 - 0.13170898365871023197e1_f64 * t16305 * t1000 + 0.13170898365871023197e1_f64 * t3063 * t4941;
    t16310
}
