//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 298/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk298<F: Float>(t1052: F, t1059: F, t1064: F, t1070: F, t1075: F, t1079: F, t1082: F, t1090: F, t1094: F) -> (F, F) {
    let t1117 = F::cast_from(0.56366309740899397906e-3_f64) * t1052 + F::cast_from(0.82073827867876094584e-5_f64) * t1059 - F::cast_from(0.11742981196020707897e-4_f64) * t1064;
    let t1125 = F::cast_from(0.27801896084645508334e-2_f64) * t1070 + F::cast_from(0.10120442708333333334e-4_f64) * t1075 - F::cast_from(0.17376185052903442709e-3_f64) * t1079 - F::cast_from(0.2318836277704281739e-4_f64) * t1082 - F::cast_from(0.84410248952307505288e-7_f64) * t1090 + F::cast_from(0.14492726735651760868e-5_f64) * t1094;
    (t1117, t1125)
}
