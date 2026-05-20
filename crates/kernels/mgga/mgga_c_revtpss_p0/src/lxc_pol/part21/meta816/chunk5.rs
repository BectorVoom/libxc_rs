//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3000/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3000<F: Float>(t15816: F, t3168: F, t10326: F, t10356: F, t1047: F, t11144: F, t11675: F, t15599: F, t15601: F, t15622: F, t1592: F, t3091: F, t3092: F, t3094: F, t3095: F, t357: F, t42410: F, t42610: F, t42965: F, t42996: F, t43003: F, t43297: F, t4583: F, t4781: F, t54026: F) -> F {
    let t54739 = t15816 * t3168;
    let t54770 = -F::cast_from(0.57165357490759649295e-3_f64) * t42965 - F::cast_from(0.13719685797782315831e-1_f64) * t43297 * t15622 - F::cast_from(0.68598428988911579154e-2_f64) * t54739 * t1047 + F::cast_from(0.19055119163586549765e-3_f64) * t42996 + F::cast_from(0.14291339372689912324e-3_f64) * t43003 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t54026 * t3095 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t3092 * t4781 * t3094 * t10326 + F::cast_from(0.63517063878621832552e-3_f64) * t3091 * t42410 * t4781 * t357 * t11144 * t10356 + F::cast_from(0.42874018118069736972e-3_f64) * t11675 * t15601 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t4583 * t15599 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t3092 * t1592 * t42610 * t357;
    t54770
}
