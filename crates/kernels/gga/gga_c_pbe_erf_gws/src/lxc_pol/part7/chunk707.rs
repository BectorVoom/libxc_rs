//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 707/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk707<F: Float>(t103: F, t2: F, t39: F, t5772: F, t102: F, t120: F, t5645: F, t506: F, t497: F, t542: F, t496: F, t10: F, t127: F, t5744: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5764: F, t5768: F, t5771: F) -> (F, F, F, F, F, F) {
    let t5773 = t103 * t2;
    let t5776 = F::cast_from(0.19486833333333333333e1_f64) * t5772 * t5773 * t39;
    let t5779 = F::cast_from(0.2923025e1_f64) * t102 * t120 * t5645;
    let t5780 = t506 * t5645;
    let t5783 = t542 * t497;
    let t5784 = t496 * t5783;
    let t5786 = F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t496 * t10 * t5744 - t5749 - t5751 + t5753 - t5755 - t5759 - F::cast_from(0.146904e1_f64) * t5764 + F::cast_from(0.220356e1_f64) * t5768 + t5771 - t5776 - t5779 - F::cast_from(0.146904e1_f64) * t127 * t5780 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5784;
    (t5773, t5776, t5779, t5780, t5783, t5786)
}
