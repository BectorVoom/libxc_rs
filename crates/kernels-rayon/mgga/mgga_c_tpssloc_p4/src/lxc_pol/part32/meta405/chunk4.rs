//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1550/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1550(t1060: f64, t18154: f64, t17959: f64, t381: f64, t1003: f64, t1058: f64, t1063: f64, t14608: f64, t1610: f64, t1632: f64, t17876: f64, t18129: f64, t18131: f64, t18139: f64, t18142: f64, t18151: f64, t3180: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4678: f64, t4681: f64, t4685: f64, t4689: f64, t4691: f64, t5903: f64, t5933: f64, t5941: f64) -> f64 {
    let t18155 = t18154 * t1060;
    let t18161 = t381 * t17959;
    let t18162 = t18161 * t1060;
    let t18164 = t1003 * t5941 + 2.0_f64 * t1058 * t18151 + t1058 * t18155 + t1058 * t18162 + t1063 * t5903 - 2.0_f64 * t14608 * t4685 + 2.0_f64 * t1610 * t4691 + 2.0_f64 * t1632 * t4615 + t17876 * t384 + t18129 * t353 - 2.0_f64 * t18131 * t3200 + 4.0_f64 * t18139 * t3186 + 4.0_f64 * t18142 * t3186 + 2.0_f64 * t3180 * t5933 + 2.0_f64 * t4669 * t4678 + 2.0_f64 * t4669 * t4681 + 2.0_f64 * t4669 * t4689;
    t18164
}
