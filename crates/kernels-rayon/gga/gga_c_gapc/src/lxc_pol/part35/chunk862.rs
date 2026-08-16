//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 862/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk862(t1081: f64, t2804: f64, t10000: f64, t10003: f64, t10006: f64, t10010: f64, t10014: f64, t10016: f64, t10019: f64, t9986: f64, t9991: f64, t9993: f64, t9995: f64, t9997: f64) -> f64 {
    let t10021 = t1081 * t2804;
    let t10023 = 0.14758978949652777778e-5_f64 * t9986 + 0.42205124476153752644e-7_f64 * t9991 - 0.1374296967252737644e-5_f64 * t9993 + 0.1081184847736214213e-1_f64 * t9995 + 0.2813674965076916843e-7_f64 * t9997 + 0.2813674965076916843e-7_f64 * t10000 - 0.13900948042322754167e-2_f64 * t10003 - 0.39192950730437765221e-2_f64 * t10006 + 0.50680539737635041234e-4_f64 * t10010 + 0.7324140771837707598e-5_f64 * t10014 + 0.75883739738679928911e-6_f64 * t10016 + 0.27801896084645508334e-2_f64 * t10019 - 0.6956508833112845217e-4_f64 * t10021;
    t10023
}
