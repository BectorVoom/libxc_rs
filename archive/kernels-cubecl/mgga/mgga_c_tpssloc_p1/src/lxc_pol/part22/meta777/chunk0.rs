//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2655/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2655<F: Float>(t12283: F, t20460: F, t20565: F, t3866: F, t1827: F, t57056: F, t20492: F, t39944: F, t12215: F, t1307: F, t16394: F, t1810: F, t19631: F, t19962: F, t19996: F, t20511: F, t210: F, t3733: F, t40025: F, t5187: F, t5226: F, t5240: F, t5259: F, t5293: F, t53882: F, t53901: F, t56878: F, t6347: F, t6370: F) -> F {
    let t74189 = t12283 * t20460;
    let t74191 = t3866 * t20565;
    let t74212 = t57056 * t1827;
    let t74214 = t39944 * t20492;
    let t74216 = -t16394 * t19962 / F::cast_from(1024.0_f64) + t56878 * t5259 / F::cast_from(256.0_f64) - t56878 * t5293 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t74189 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t74191 - t53882 + F::cast_from(595.0_f64) / F::cast_from(864.0_f64) * t53901 + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t40025 * t210 * t20511 * t1307 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12215 * t210 * t6370 * t5187 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3733 * t210 * t5226 * t6347 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3733 * t210 * t1810 * t19631 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t5240 * t19996 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t74212 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t74214;
    t74216
}
