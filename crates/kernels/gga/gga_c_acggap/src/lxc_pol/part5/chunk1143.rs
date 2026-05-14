//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1143/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1143<F: Float>(t4389: F, t5859: F, t1181: F, t12991: F, t4347: F, t530: F, t4396: F, t6332: F, t12930: F, t1761: F, t3409: F, t5807: F, t6153: F, t15482: F, t6339: F, t1165: F, t1531: F, t1533: F, t18153: F, t18155: F, t18157: F, t20906: F, t3084: F, t5862: F) -> (F,) {
    let t23568 = t4389 * t5859;
    let t23572 = t12991 * t1181 * t530 * t4347;
    let t23574 = t4396 * t6332;
    let t23584 = t12930 * t1761;
    let t23586 = t3409 * t5807;
    let t23588 = t3409 * t6153;
    let t23590 = t15482 * t6339;
    let t23592 = -0.17149607247227894789e-2 * t18153 + 0.17149607247227894789e-2 * t18155 - 0.85748036236139473944e-3 * t18157 - 0.12004725073059526352e-1 * t23568 - 0.68598428988911579156e-2 * t23572 + 0.85748036236139473944e-3 * t23574 + 0.85748036236139473944e-3 * t1531 * t1165 * t20906 * t1533 + 0.42874018118069736972e-3 * t1531 * t1165 * t5862 * t3084 - 0.80031500487063509014e-2 * t23584 - 0.80031500487063509014e-2 * t23586 - 0.80031500487063509014e-2 * t23588 - 0.48018900292238105409e-1 * t23590;
    (t23592,)
}
