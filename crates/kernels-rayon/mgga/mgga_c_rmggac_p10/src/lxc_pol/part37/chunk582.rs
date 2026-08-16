//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 582/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk582(t15078: f64, t1550: f64, t291: f64, t8465: f64, t13823: f64, t3080: f64, t570: f64, t5148: f64, t551: f64, t5259: f64, t558: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15079 = t1550 * t15078;
    let t15081 = t8465 * t291;
    let t15082 = t13823 * t15081;
    let t15084 = t3080 * t570;
    let t15086 = 0.5987120850931904282e-1_f64 * t5148 * t15084;
    let t15087 = t3080 * t551;
    let t15089 = 0.5987120850931904282e-1_f64 * t5259 * t15087;
    let t15090 = t3080 * t558;
    let t15092 = 0.8980681276397856423e-1_f64 * t4669 * t15090;
    (t15079, t15081, t15082, t15084, t15086, t15087, t15089, t15090, t15092)
}
