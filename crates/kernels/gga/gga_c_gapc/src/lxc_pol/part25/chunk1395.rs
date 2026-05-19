//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1395/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1395<F: Float>(t34715: F, t34718: F, t34720: F, t34723: F, t34726: F, t34729: F, t34732: F, t34735: F, t34745: F, t34747: F, t34749: F, t34752: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37047 = F::cast_from(0.26681999992642267404e-7_f64) * t34715;
    let t37048 = F::cast_from(0.2318836277704281739e-4_f64) * t34718;
    let t37049 = F::cast_from(0.3243554543208642639e-2_f64) * t34720;
    let t37050 = F::cast_from(0.69504740211613770836e-3_f64) * t34723;
    let t37051 = F::cast_from(0.69504740211613770836e-3_f64) * t34726;
    let t37052 = F::cast_from(0.34752370105806885418e-3_f64) * t34729;
    let t37053 = F::cast_from(0.67402122125348062552e-7_f64) * t34732;
    let t37054 = F::cast_from(0.11372686522837130914e-5_f64) * t34735;
    let t37058 = F::cast_from(0.13506074236995523433e-5_f64) * t34745;
    let t37059 = F::cast_from(0.1011909669415296852e-6_f64) * t34747;
    let t37060 = F::cast_from(0.15458908518028544927e-5_f64) * t34749;
    let t37061 = F::cast_from(0.80966145833333333338e-4_f64) * t34752;
    (t37047, t37048, t37049, t37050, t37051, t37052, t37053, t37054, t37058, t37059, t37060, t37061)
}
