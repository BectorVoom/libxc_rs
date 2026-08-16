//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 978/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk978(t2749: f64, t4167: f64, t840: f64, t4129: f64, t875: f64, t871: f64, t1212: f64, t2801: f64, t1882: f64, t4164: f64, t4169: f64, t2862: f64, t4162: f64, t882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15154 = t840 * t2749 * t4167;
    let t15157 = t4129 * t875;
    let t15159 = t840 * t871 * t15157;
    let t15162 = t1212 * t2801;
    let t15164 = t840 * t871 * t15162;
    let t15168 = 4.0_f64 / 9.0_f64 * t1882 * t4164;
    let t15170 = 2.0_f64 / 9.0_f64 * t1882 * t4169;
    let t15172 = t2862 * t882 * t4162;
    (t15154, t15159, t15164, t15168, t15170, t15172)
}
