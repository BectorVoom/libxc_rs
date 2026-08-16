//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 422/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk422(t3208: f64, t5: f64, t1016: f64, t119: f64, t4: f64, t181: f64, t944: f64, t3088: f64, t1021: f64, t3107: f64, t1011: f64, t1015: f64, t1018: f64, t1022: f64, t11: f64, t139: f64, t157: f64, t175: f64, t197: f64, t198: f64, t201: f64, t3125: f64, t3190: f64, t3194: f64, t3200: f64, t3203: f64, t3207: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3209 = t5 * t3208;
    let t3213 = t1016 * t4 * t119;
    let t3216 = t181 * t944;
    let t3217 = t3216 * t3088;
    let t3220 = t1021 * t3107;
    let t3232 = 0.619125e-2_f64 * t3190 * t198 - 0.24765e-1_f64 * t3194 * t1018 - 0.123825e-1_f64 * t1011 * t1022 + 0.206375e-2_f64 * t3200 * t3203 + 0.24765e-1_f64 * t3207 * t3209 + 0.1651e-1_f64 * t1015 * t3213 + 0.123825e-1_f64 * t197 * t3217 - 0.619125e-2_f64 * t197 * t3220 + 0.17687407407407407407e-1_f64 * t139 * t157 * t175 - 0.10612444444444444444e0_f64 * t139 * t11 * t972 - 0.79593333333333333331e-1_f64 * t139 * t201 * t3125;
    (t3209, t3213, t3216, t3217, t3220, t3232)
}
