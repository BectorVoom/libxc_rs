//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2606/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2606(t1892: f64, t3923: f64, t2782: f64, t4003: f64, t5744: f64, t10069: f64, t14124: f64, t14129: f64, t14231: f64, t14255: f64, t4057: f64, t46443: f64, t46448: f64, t46452: f64, t46454: f64, t46458: f64, t47971: f64, t820: f64) -> (f64, f64) {
    let t47973 = t1892 * t3923;
    let t47976 = t2782 * t5744 * t47973 * t4003;
    let t47978 = t10069 * t14124;
    let t47979 = 0.21951497276451705329e-1_f64 * t47978;
    let t47980 = t10069 * t14129;
    let t47981 = 0.21951497276451705329e-1_f64 * t47980;
    let t47985 = t10069 * t14231;
    let t47992 = 0.30356481678079769392e-1_f64 * t47971 - 0.32927245914677557992e-1_f64 * t47976 - t47979 - t47981 - 0.19756347548806534796e1_f64 * t820 * t14255 * t4057 + 0.43902994552903410656e-1_f64 * t47985 + 0.78059524315062264151e-1_f64 * t46443 + 0.39029762157531132075e-1_f64 * t46448 - 0.78059524315062264151e-1_f64 * t46452 - 0.29272321618148349057e-1_f64 * t46454 + 0.58544643236296698114e-1_f64 * t46458;
    (t47973, t47992)
}
