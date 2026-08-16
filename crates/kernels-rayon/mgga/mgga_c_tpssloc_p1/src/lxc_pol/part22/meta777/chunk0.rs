//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2655/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2655(t12283: f64, t20460: f64, t20565: f64, t3866: f64, t1827: f64, t57056: f64, t20492: f64, t39944: f64, t12215: f64, t1307: f64, t16394: f64, t1810: f64, t19631: f64, t19962: f64, t19996: f64, t20511: f64, t210: f64, t3733: f64, t40025: f64, t5187: f64, t5226: f64, t5240: f64, t5259: f64, t5293: f64, t53882: f64, t53901: f64, t56878: f64, t6347: f64, t6370: f64) -> f64 {
    let t74189 = t12283 * t20460;
    let t74191 = t3866 * t20565;
    let t74212 = t57056 * t1827;
    let t74214 = t39944 * t20492;
    let t74216 = -t16394 * t19962 / 1024.0_f64 + t56878 * t5259 / 256.0_f64 - t56878 * t5293 / 1024.0_f64 - 7.0_f64 / 384.0_f64 * t74189 - 35.0_f64 / 384.0_f64 * t74191 - t53882 + 595.0_f64 / 864.0_f64 * t53901 + 5.0_f64 / 4.0_f64 * t40025 * t210 * t20511 * t1307 - 3.0_f64 / 4.0_f64 * t12215 * t210 * t6370 * t5187 + 3.0_f64 / 16.0_f64 * t3733 * t210 * t5226 * t6347 + 3.0_f64 / 16.0_f64 * t3733 * t210 * t1810 * t19631 + 5.0_f64 / 256.0_f64 * t5240 * t19996 + 7.0_f64 / 1536.0_f64 * t74212 + 7.0_f64 / 768.0_f64 * t74214;
    t74216
}
