//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 622/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk622<F: Float>(t1449: F, t4267: F, t1181: F, t3539: F, t5852: F, t1165: F, t3463: F, t1150: F, t1173: F, t1180: F, t1531: F, t335: F, t3358: F, t3373: F, t3376: F, t3396: F, t3428: F, t3462: F, t367: F, t4463: F, t4627: F, t5884: F, t5891: F, t5895: F, t5899: F, t5903: F, t5907: F, t5910: F, t5913: F, t5916: F, t5919: F, t5924: F, t5928: F) -> (F, F, F, F) {
    let t5931 = t4267 * t1449;
    let t5932 = t1181 * t5931;
    let t5936 = t1181 * t5852 * t3539;
    let t5940 = t1165 * t5852 * t3463;
    let t5943 = -F::new(7.0) / F::new(288.0) * t5884 - F::new(0.17149607247227894789e-2) * t3358 - F::new(0.20007875121765877254e-2) * t3373 + F::new(0.42874018118069736972e-3) * t3376 + F::new(0.21437009059034868486e-3) * t3428 + F::new(0.85748036236139473944e-3) * t1173 * t5891 - F::new(0.42874018118069736972e-3) * t1180 * t5895 + F::new(0.42874018118069736972e-3) * t1180 * t5899 + F::new(0.68598428988911579156e-2) * t3396 * t5903 - t4627 + t335 * t5907 / F::new(48.0) + t335 * t5910 / F::new(24.0) + t367 * t5913 / F::new(24.0) + t1150 * t5916 / F::new(8.0) + t335 * t5919 / F::new(24.0) - F::new(0.85748036236139473944e-3) * t1531 * t5924 + F::new(0.17149607247227894789e-1) * t4463 * t5928 + F::new(0.68598428988911579156e-2) * t3396 * t5932 + F::new(0.85748036236139473944e-3) * t1531 * t5936 - F::new(0.17149607247227894789e-2) * t3462 * t5940;
    (t5932, t5936, t5940, t5943)
}
