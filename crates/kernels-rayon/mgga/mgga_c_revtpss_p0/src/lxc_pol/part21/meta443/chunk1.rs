//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1963/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1963(t14188: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64, t10062: f64, t10130: f64, t13805: f64, t1399: f64, t14122: f64, t14127: f64, t14158: f64, t14161: f64, t14166: f64, t14171: f64, t1883: f64, t3924: f64, t4004: f64, t4057: f64, t5675: f64, t5735: f64, t5745: f64, t5755: f64, t5767: f64, t820: f64) -> (f64, f64, f64, f64) {
    let t14189 = t869 * t14188;
    let t14191 = 0.10975748638225852664e-1_f64 * t689 * t14189;
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    let t14200 = -0.65854491829355115987e0_f64 * t820 * t10130 * t1883 + t14158 + 0.11565819519348392139e-2_f64 * t14161 - 0.65854491829355115987e0_f64 * t5755 * t5735 * t3924 + 0.73171657588172351096e-2_f64 * t14166 - 0.65854491829355115987e0_f64 * t820 * t5767 * t4057 + 0.13170898365871023197e1_f64 * t820 * t14171 * t4004 + 0.26341796731742046394e1_f64 * t5745 * t14122 * t5675 + 0.26341796731742046394e1_f64 * t5745 * t14127 * t5675 - 0.13170898365871023197e1_f64 * t5755 * t14122 * t1399 - 0.13170898365871023197e1_f64 * t5755 * t14127 * t1399 - 0.10975748638225852664e-1_f64 * t10062 - t14191 - 0.39512695097613069591e1_f64 * t14193 * t5735 * t13805 - 0.65854491829355115987e0_f64 * t5755 * t5735 * t4057;
    (t14189, t14192, t14193, t14200)
}
