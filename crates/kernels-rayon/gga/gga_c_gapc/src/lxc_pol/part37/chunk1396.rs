//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1396/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1396(t34715: f64, t34718: f64, t34720: f64, t34723: f64, t34726: f64, t34729: f64, t34732: f64, t34735: f64, t34745: f64, t34747: f64, t34749: f64, t34752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37047 = 0.26681999992642267404e-7_f64 * t34715;
    let t37048 = 0.2318836277704281739e-4_f64 * t34718;
    let t37049 = 0.3243554543208642639e-2_f64 * t34720;
    let t37050 = 0.69504740211613770836e-3_f64 * t34723;
    let t37051 = 0.69504740211613770836e-3_f64 * t34726;
    let t37052 = 0.34752370105806885418e-3_f64 * t34729;
    let t37053 = 0.67402122125348062552e-7_f64 * t34732;
    let t37054 = 0.11372686522837130914e-5_f64 * t34735;
    let t37058 = 0.13506074236995523433e-5_f64 * t34745;
    let t37059 = 0.1011909669415296852e-6_f64 * t34747;
    let t37060 = 0.15458908518028544927e-5_f64 * t34749;
    let t37061 = 0.80966145833333333338e-4_f64 * t34752;
    (t37047, t37048, t37049, t37050, t37051, t37052, t37053, t37054, t37058, t37059, t37060, t37061)
}
