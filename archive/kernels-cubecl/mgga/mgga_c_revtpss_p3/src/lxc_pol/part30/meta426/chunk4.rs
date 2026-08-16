//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1623/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1623<F: Float>(t15717: F, t996: F, t1678: F, t3057: F, t15648: F, t16152: F, t15837: F, t4930: F, t994: F, t3046: F, t1000: F, t11187: F, t11201: F, t11220: F, t1680: F, t1696: F, t3043: F, t3047: F, t3058: F, t3060: F, t3063: F, t3264: F, t3271: F, t4752: F, t4758: F, t4764: F, t4773: F, t4941: F, t4947: F, t995: F) -> F {
    let t16275 = t996 * t15717;
    let t16284 = t3057 * t1678;
    let t16287 = t996 * t15648;
    let t16292 = t996 * t16152;
    let t16295 = t996 * t15837;
    let t16302 = t994 * t4930;
    let t16305 = t3046 * t1678;
    let t16310 = F::cast_from(0.13170898365871023197e1_f64) * t3047 * t4764 - F::cast_from(0.39512695097613069591e1_f64) * t11201 * t16275 + F::cast_from(0.26341796731742046394e1_f64) * t3264 * t4947 - F::cast_from(0.13170898365871023197e1_f64) * t11220 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t4752 * t3271 + F::cast_from(0.13170898365871023197e1_f64) * t16284 * t3060 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t16287 + F::cast_from(0.26341796731742046394e1_f64) * t11187 * t4758 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t16292 + F::cast_from(0.13170898365871023197e1_f64) * t3058 * t16295 + F::cast_from(0.65854491829355115987e0_f64) * t3043 * t1680 - F::cast_from(0.13170898365871023197e1_f64) * t3063 * t4773 - F::cast_from(0.13170898365871023197e1_f64) * t16302 * t1000 - F::cast_from(0.13170898365871023197e1_f64) * t16305 * t1000 + F::cast_from(0.13170898365871023197e1_f64) * t3063 * t4941;
    t16310
}
