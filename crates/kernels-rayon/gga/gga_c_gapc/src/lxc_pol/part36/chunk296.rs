//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 296/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk296(t1052: f64, t1059: f64, t1064: f64, t1070: f64, t1075: f64, t1079: f64, t1082: f64, t1090: f64, t1094: f64) -> (f64, f64) {
    let t1117 = 0.56366309740899397906e-3_f64 * t1052 + 0.82073827867876094584e-5_f64 * t1059 - 0.11742981196020707897e-4_f64 * t1064;
    let t1125 = 0.27801896084645508334e-2_f64 * t1070 + 0.10120442708333333334e-4_f64 * t1075 - 0.17376185052903442709e-3_f64 * t1079 - 0.2318836277704281739e-4_f64 * t1082 - 0.84410248952307505288e-7_f64 * t1090 + 0.14492726735651760868e-5_f64 * t1094;
    (t1117, t1125)
}
