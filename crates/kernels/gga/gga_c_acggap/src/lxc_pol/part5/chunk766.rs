//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 766/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk766<F: Float>(t1841: F, t952: F, t1846: F, t935: F, t1180: F, t127: F, t3246: F, t3312: F, t3314: F, t418: F, t4492: F, t4494: F, t4505: F, t5787: F, t5790: F, t5792: F, t5796: F, t5801: F, t5804: F, t5807: F, t5811: F, t5816: F, t5821: F, t5827: F, t5829: F) -> F {
    let t5831 = t952 * t1841;
    let t5833 = t935 * t1846;
    let t5837 = -t3246 + t127 * t5787 / F::new(96.0) - t4492 - t4494 - F::cast_from(0.85748036236139473944e-3_f64) * t5790 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t5792 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t5796 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t5801 + F::cast_from(0.85748036236139473944e-3_f64) * t5804 + F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t5807 - t4505 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t5811 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t5816 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t5821 - F::cast_from(0.42874018118069736972e-2_f64) * t5827 - F::cast_from(0.21437009059034868486e-3_f64) * t5829 + F::cast_from(0.10003937560882938627e-2_f64) * t5831 - F::cast_from(0.21437009059034868486e-3_f64) * t5833 + F::cast_from(0.17149607247227894789e-2_f64) * t3312 - F::cast_from(0.85748036236139473944e-3_f64) * t3314;
    t5837
}
