//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 929/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk929<F: Float>(t496: F, t8146: F, t1243: F, t2890: F, t128: F, t8102: F, t10: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5764: F, t5768: F, t5776: F, t8117: F, t8118: F, t8126: F, t8127: F, t8131: F, t8137: F, t8139: F, t8142: F, t8145: F) -> (F, F) {
    let t8148 = t496 * t8146 / F::cast_from(3.0_f64);
    let t8149 = t2890 * t1243;
    let t8151 = t128 * t8102;
    let t8152 = t10 * t8151;
    let t8155 = -t5749 - t5751 + t5753 - t5755 - t5759 - F::cast_from(0.97936000000000000001e0_f64) * t5764 + F::cast_from(0.73452e0_f64) * t5768 + t8117 - t5776 - F::cast_from(6.0_f64) * t496 * t10 * t8118 - t8126 + F::cast_from(3.0_f64) * t496 * t10 * t8127 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t496 * t10 * t8131 - t8137 - t8139 + t8142 + t8145 + t8148 - F::cast_from(0.97936e0_f64) * t8149 - t496 * t8152 / F::cast_from(2.0_f64);
    (t8152, t8155)
}
