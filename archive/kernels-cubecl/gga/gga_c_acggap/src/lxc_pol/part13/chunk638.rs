//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 638/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk638<F: Float>(t1426: F, t175: F, t4822: F, t1462: F, t997: F, t1173: F, t1180: F, t3403: F, t397: F, t418: F, t4946: F, t4949: F, t4950: F, t4953: F, t4954: F, t4957: F, t4961: F, t4963: F, t4969: F, t4971: F, t4975: F, t4979: F, t4983: F, t4989: F, t4991: F, t4994: F, t4996: F, t4999: F) -> (F, F) {
    let t5003 = t1426 * t175 * t4822;
    let t5007 = F::cast_from(0.12004725073059526352e-1_f64) * t997 * t1462;
    let t5008 = -F::cast_from(0.85748036236139473944e-3_f64) * t4946 + t4949 + F::cast_from(0.40015750243531754508e-2_f64) * t4950 - t4953 - F::cast_from(0.40015750243531754508e-2_f64) * t4954 - t4957 + t4961 - F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t4963 - t4969 - F::cast_from(0.85748036236139473944e-2_f64) * t3403 * t4971 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t4975 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t4979 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t4983 - t4989 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t4991 + F::cast_from(0.17149607247227894789e-2_f64) * t4994 - F::cast_from(0.56688979511669985553e-2_f64) * t4996 - F::cast_from(0.42874018118069736972e-3_f64) * t397 * t4999 + F::cast_from(0.42874018118069736972e-2_f64) * t418 * t5003 - t5007;
    (t5003, t5008)
}
