//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 770/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk770(t13064: f64, t969: f64, t825: f64, t2685: f64, t2684: f64, t3247: f64, t900: f64, t10867: f64, t10924: f64, t787: f64, t9824: f64, t12555: f64, t12558: f64, t12561: f64, t12564: f64, t12566: f64, t12569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13065 = t969 * t13064;
    let t13066 = t825 * t13065;
    let t13069 = t2685 * t13064;
    let t13070 = t2684 * t13069;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13074 = 0.89376224879626066675e-1_f64 * t13073;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = 0.29792074959875355558e-1_f64 * t13078;
    let t13086 = -3.0_f64 / 256.0_f64 * t12555 - 27.0_f64 / 8192.0_f64 * t12558 + 27.0_f64 / 524288.0_f64 * t12561 - 9.0_f64 / 524288.0_f64 * t12564 + 9.0_f64 / 8192.0_f64 * t12566 + t12569 / 256.0_f64;
    (t13065, t13066, t13069, t13070, t13072, t13074, t13077, t13079, t13086)
}
