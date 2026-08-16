//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 950/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk950(t12983: f64, t1375: f64, t12868: f64, t457: f64, t1383: f64, t1186: f64, t13456: f64, t14047: f64, t14056: f64, t14059: f64, t14062: f64, t14063: f64, t158: f64, t165: f64, t173: f64, t3819: f64, t3891: f64) -> f64 {
    let t14066 = t1375 * t12983;
    let t14069 = t1375 * t12868;
    let t14072 = t457 * t12983;
    let t14075 = t1383 * t12868;
    let t14078 = t1186 * t12983;
    let t14081 = 0.403305e-4_f64 * t173 * t14047 - 0.71734315950379065738e-1_f64 * t3819 * t13456 + 0.46615850170166761884e-3_f64 * t3891 * t13456 + t14056 + t14059 - t14062 - 0.30247875e-4_f64 * t173 * t14063 - 0.2016525e-4_f64 * t173 * t14066 + 0.21078e-1_f64 * t158 * t14069 + 0.3513e-2_f64 * t158 * t14072 - 0.4755e-2_f64 * t165 * t14075 - 0.1585e-2_f64 * t165 * t14078;
    t14081
}
