//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1293/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1293(t1394: f64, t8164: f64, t98020: f64, t28331: f64, t28356: f64, t5780: f64, t27364: f64, t29368: f64, t1014: f64, t29401: f64, t102102: f64, t102106: f64, t102109: f64, t102115: f64, t102118: f64, t8213: f64, t94979: f64, t99260: f64, t99497: f64) -> (f64, f64, f64, f64, f64) {
    let t102121 = t1394 * t98020 * t8164;
    let t102124 = t5780 * t28356 * t28331;
    let t102127 = t1394 * t27364 * t29368;
    let t102129 = t1014 * t29401;
    let t102131 = -0.10306077835648148148e-4_f64 * t94979 - 0.38691203703703703703e-3_f64 * t102102 + t99260 + 0.69644166666666666664e-2_f64 * t102106 - 0.23214722222222222221e-2_f64 * t102109 + 0.92754700520833333334e-4_f64 * t99497 * t8213 - 0.18571777777777777777e-1_f64 * t102115 + 0.61905925925925925924e-2_f64 * t102118 - 0.61905925925925925925e-2_f64 * t102121 + 0.12381185185185185185e-1_f64 * t102124 - 0.23214722222222222222e-2_f64 * t102127 + 0.11349419753086419753e-1_f64 * t102129;
    (t102121, t102124, t102127, t102129, t102131)
}
