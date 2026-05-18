//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1275/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1275<F: Float>(t3409: F, t6153: F, t15482: F, t6339: F, t1165: F, t1531: F, t1533: F, t18153: F, t18155: F, t18157: F, t20906: F, t23568: F, t23572: F, t23574: F, t23584: F, t23586: F, t3084: F, t5862: F) -> F {
    let t23588 = t3409 * t6153;
    let t23590 = t15482 * t6339;
    let t23592 = -F::new(0.17149607247227894789e-2) * t18153 + F::new(0.17149607247227894789e-2) * t18155 - F::new(0.85748036236139473944e-3) * t18157 - F::new(0.12004725073059526352e-1) * t23568 - F::new(0.68598428988911579156e-2) * t23572 + F::new(0.85748036236139473944e-3) * t23574 + F::new(0.85748036236139473944e-3) * t1531 * t1165 * t20906 * t1533 + F::new(0.42874018118069736972e-3) * t1531 * t1165 * t5862 * t3084 - F::new(0.80031500487063509014e-2) * t23584 - F::new(0.80031500487063509014e-2) * t23586 - F::new(0.80031500487063509014e-2) * t23588 - F::new(0.48018900292238105409e-1) * t23590;
    t23592
}
