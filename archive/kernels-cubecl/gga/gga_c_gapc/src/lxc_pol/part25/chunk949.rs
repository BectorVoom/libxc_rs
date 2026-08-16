//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 949/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk949<F: Float>(t9857: F, t9860: F, t9866: F, t9869: F, t9872: F, t9874: F, t9876: F, t9878: F, t9881: F, t9883: F, t9885: F, t9887: F, t9889: F) -> F {
    let t10975 = -F::cast_from(0.34752370105806885418e-3_f64) * t9857 + F::cast_from(0.51491428373437201896e-5_f64) * t9860 + F::cast_from(0.98478623777692089505e-7_f64) * t9866 + F::cast_from(0.34752370105806885418e-3_f64) * t9869 + F::cast_from(0.17376185052903442709e-3_f64) * t9872 + F::cast_from(0.4637672555408563478e-4_f64) * t9874 - F::cast_from(0.30353495895471971564e-6_f64) * t9876 + F::cast_from(0.53968515702149165441e-6_f64) * t9878 - F::cast_from(0.46497498276882732785e-5_f64) * t9881 + F::cast_from(0.43284943850479925795e-3_f64) * t9883 - F::cast_from(0.43284943850479925795e-3_f64) * t9885 - F::cast_from(0.41223756048076119805e-5_f64) * t9887 + F::cast_from(0.73295838253479341016e-5_f64) * t9889;
    t10975
}
