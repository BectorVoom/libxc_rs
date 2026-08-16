//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 701/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk701(t13056: f64, t1029: f64, t3295: f64, t9796: f64, t3247: f64, t900: f64, t10867: f64, t10924: f64, t787: f64, t9824: f64, t12555: f64, t12558: f64, t12561: f64, t12564: f64, t12566: f64, t12569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13057 = 0.11502877786176224903e1_f64 * t13056;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = 0.29792074959875355558e-1_f64 * t13078;
    let t13086 = -3.0_f64 / 256.0_f64 * t12555 - 27.0_f64 / 8192.0_f64 * t12558 + 27.0_f64 / 524288.0_f64 * t12561 - 9.0_f64 / 524288.0_f64 * t12564 + 9.0_f64 / 8192.0_f64 * t12566 + t12569 / 256.0_f64;
    (t13057, t13058, t13059, t13072, t13073, t13077, t13079, t13086)
}
