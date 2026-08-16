//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 929/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk929(t496: f64, t8146: f64, t1243: f64, t2890: f64, t128: f64, t8102: f64, t10: f64, t5749: f64, t5751: f64, t5753: f64, t5755: f64, t5759: f64, t5764: f64, t5768: f64, t5776: f64, t8117: f64, t8118: f64, t8126: f64, t8127: f64, t8131: f64, t8137: f64, t8139: f64, t8142: f64, t8145: f64) -> (f64, f64) {
    let t8148 = t496 * t8146 / 3.0_f64;
    let t8149 = t2890 * t1243;
    let t8151 = t128 * t8102;
    let t8152 = t10 * t8151;
    let t8155 = -t5749 - t5751 + t5753 - t5755 - t5759 - 0.97936000000000000001e0_f64 * t5764 + 0.73452e0_f64 * t5768 + t8117 - t5776 - 6.0_f64 * t496 * t10 * t8118 - t8126 + 3.0_f64 * t496 * t10 * t8127 + 3.0_f64 / 2.0_f64 * t496 * t10 * t8131 - t8137 - t8139 + t8142 + t8145 + t8148 - 0.97936e0_f64 * t8149 - t496 * t8152 / 2.0_f64;
    (t8152, t8155)
}
