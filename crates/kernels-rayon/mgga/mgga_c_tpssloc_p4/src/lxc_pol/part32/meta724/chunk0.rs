//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2318/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2318(t18329: f64, t7310: f64, t1734: f64, t18303: f64, t18948: f64, t18955: f64, t19058: f64, t19062: f64, t19072: f64, t19077: f64, t24729: f64, t24733: f64, t27604: f64, t27617: f64, t478: f64, t4974: f64, t4980: f64, t4984: f64, t4989: f64, t7345: f64, t7376: f64, t86146: f64, t86171: f64, t95270: f64, t95273: f64, t95303: f64, t95304: f64) -> f64 {
    let t104085 = t7310 * t18329;
    let t104087 = t24729 * t19058 / 768.0_f64 - t24733 * t19062 / 1536.0_f64 + t95270 * t4980 / 384.0_f64 - t95273 * t4984 / 768.0_f64 + 5.0_f64 / 3456.0_f64 * t27617 * t4989 + t24729 * t18948 / 384.0_f64 + t86146 * t18303 / 256.0_f64 + t27604 * t4974 / 108.0_f64 - 5.0_f64 / 2592.0_f64 * t7345 * t18955 - t24733 * t19072 / 768.0_f64 + t86171 * t19077 / 1536.0_f64 - 0.20186378047070195428e-3_f64 * t95303 * t95304 * t478 * t1734 * t7376 - t104085 / 864.0_f64;
    t104087
}
