//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1363/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1363<F: Float>(t3717: F, t3754: F, t103101: F, t5701: F, t103239: F, t7908: F, t28372: F, t52697: F, t5885: F, t16937: F, t29258: F, t102158: F, t102180: F, t102183: F, t12194: F, t16901: F, t16906: F, t20984: F, t21655: F, t27369: F, t27438: F, t59414: F, t94408: F, t94519: F) -> (F, F) {
    let t103372 = t3717 * t3754;
    let t103374 = t5701 * t103372 * t103101;
    let t103391 = t7908 * t103239;
    let t103394 = t28372 * t5885 * t52697;
    let t103399 = t7908 * t16937 * t29258;
    let t103402 = -F::new(0.55273148148148148147e-2) * t102158 + F::new(0.30891203703703703704e-3) * t7908 * t12194 * t27438 * t59414 + F::new(0.30891203703703703704e-3) * t7908 * t103374 + F::new(0.18534722222222222223e-2) * t7908 * t5701 * t94519 * t20984 + F::new(0.41224311342592592593e-4) * t27369 * t103374 - F::new(0.72079475308641975309e-3) * t7908 * t16901 * t94408 * t20984 + F::new(0.12356481481481481482e-2) * t7908 * t16906 * t27438 * t21655 + F::new(0.15445601851851851852e-3) * t103391 + F::new(0.37101880208333333334e-3) * t27369 * t103394 - F::new(0.22109259259259259259e-2) * t102180 + F::new(0.15445601851851851852e-3) * t103399 - F::new(0.33163888888888888888e-2) * t102183;
    (t103394, t103402)
}
