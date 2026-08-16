//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1323/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1323<F: Float>(t35169: F, t35173: F, t35177: F, t35182: F, t35184: F, t35186: F, t35188: F, t35190: F, t35192: F, t35197: F, t35200: F, t35203: F, t35205: F, t35210: F, t35212: F, t35215: F, t35217: F, t35222: F, t35225: F, t35228: F, t35231: F, t35234: F) -> (F, F) {
    let t38435 = F::cast_from(0.23968194627773771045e-6_f64) * t35169 + F::cast_from(0.12670134934408760308e-4_f64) * t35173 - F::cast_from(0.21851722570348668985e-8_f64) * t35177 - F::cast_from(0.77055513242940134824e-7_f64) * t35182 - F::cast_from(0.13900948042322754167e-2_f64) * t35184 - F::cast_from(0.69504740211613770836e-3_f64) * t35186 + F::cast_from(0.6956508833112845217e-4_f64) * t35188 - F::cast_from(0.62867981369975898391e-7_f64) * t35190 - F::cast_from(0.13526544953274976811e-4_f64) * t35192 + F::cast_from(0.24289843750000000002e-2_f64) * t35197 - F::cast_from(0.28464357592150770868e-7_f64) * t35200;
    let t38447 = F::cast_from(0.14492726735651760868e-5_f64) * t35203 + F::cast_from(0.5497187869010950576e-5_f64) * t35205 + F::cast_from(0.36954560225358884233e-5_f64) * t35210 - F::cast_from(0.16038463156432184077e-5_f64) * t35212 - F::cast_from(0.16009199995585360443e-6_f64) * t35215 + F::cast_from(0.20596571349374880758e-4_f64) * t35217 - F::cast_from(0.30036666551171701105e-5_f64) * t35222 + F::cast_from(0.12141398358188788626e-5_f64) * t35225 + F::cast_from(0.86880925264517213544e-4_f64) * t35228 + F::cast_from(0.22745373045674261828e-5_f64) * t35231 - F::cast_from(0.21135226489492151266e-6_f64) * t35234;
    (t38435, t38447)
}
