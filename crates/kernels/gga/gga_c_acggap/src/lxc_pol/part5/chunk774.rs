//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 774/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk774<F: Float>(t1165: F, t3544: F, t5922: F, t1444: F, t530: F, t1181: F, t1449: F, t4267: F, t3539: F, t5852: F, t3463: F, t1150: F, t1173: F, t1180: F, t1531: F, t335: F, t3358: F, t3373: F, t3376: F, t3396: F, t3428: F, t3462: F, t367: F, t4463: F, t4627: F, t5884: F, t5891: F, t5895: F, t5899: F, t5903: F, t5907: F, t5910: F, t5913: F, t5916: F, t5919: F) -> (F, F, F, F, F, F, F, F) {
    let t5924 = t1165 * t5922 * t3544;
    let t5927 = t530 * t1444;
    let t5928 = t1181 * t5927;
    let t5931 = t4267 * t1449;
    let t5932 = t1181 * t5931;
    let t5936 = t1181 * t5852 * t3539;
    let t5940 = t1165 * t5852 * t3463;
    let t5943 = -F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t5884 - F::cast_from(0.17149607247227894789e-2_f64) * t3358 - F::cast_from(0.20007875121765877254e-2_f64) * t3373 + F::cast_from(0.42874018118069736972e-3_f64) * t3376 + F::cast_from(0.21437009059034868486e-3_f64) * t3428 + F::cast_from(0.85748036236139473944e-3_f64) * t1173 * t5891 - F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t5895 + F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t5899 + F::cast_from(0.68598428988911579156e-2_f64) * t3396 * t5903 - t4627 + t335 * t5907 / F::cast_from(48.0_f64) + t335 * t5910 / F::cast_from(24.0_f64) + t367 * t5913 / F::cast_from(24.0_f64) + t1150 * t5916 / F::cast_from(8.0_f64) + t335 * t5919 / F::cast_from(24.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t5924 + F::cast_from(0.17149607247227894789e-1_f64) * t4463 * t5928 + F::cast_from(0.68598428988911579156e-2_f64) * t3396 * t5932 + F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t5936 - F::cast_from(0.17149607247227894789e-2_f64) * t3462 * t5940;
    (t5924, t5927, t5928, t5931, t5932, t5936, t5940, t5943)
}
