//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2606/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2606<F: Float>(t1892: F, t3923: F, t2782: F, t4003: F, t5744: F, t10069: F, t14124: F, t14129: F, t14231: F, t14255: F, t4057: F, t46443: F, t46448: F, t46452: F, t46454: F, t46458: F, t47971: F, t820: F) -> (F, F) {
    let t47973 = t1892 * t3923;
    let t47976 = t2782 * t5744 * t47973 * t4003;
    let t47978 = t10069 * t14124;
    let t47979 = F::cast_from(0.21951497276451705329e-1_f64) * t47978;
    let t47980 = t10069 * t14129;
    let t47981 = F::cast_from(0.21951497276451705329e-1_f64) * t47980;
    let t47985 = t10069 * t14231;
    let t47992 = F::cast_from(0.30356481678079769392e-1_f64) * t47971 - F::cast_from(0.32927245914677557992e-1_f64) * t47976 - t47979 - t47981 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t14255 * t4057 + F::cast_from(0.43902994552903410656e-1_f64) * t47985 + F::cast_from(0.78059524315062264151e-1_f64) * t46443 + F::cast_from(0.39029762157531132075e-1_f64) * t46448 - F::cast_from(0.78059524315062264151e-1_f64) * t46452 - F::cast_from(0.29272321618148349057e-1_f64) * t46454 + F::cast_from(0.58544643236296698114e-1_f64) * t46458;
    (t47973, t47992)
}
