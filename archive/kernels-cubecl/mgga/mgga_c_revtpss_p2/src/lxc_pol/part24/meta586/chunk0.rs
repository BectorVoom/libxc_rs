//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1821/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1821<F: Float>(t74130: F, t74132: F, t48313: F, t85986: F, t40067: F, t40072: F, t47096: F, t47098: F, t47109: F, t47116: F, t47118: F, t47122: F) -> (F, F, F, F, F) {
    let t92019 = F::cast_from(0.70178683471615754484e1_f64) * t74130;
    let t92020 = F::cast_from(48.0_f64) * t74132;
    let t92021 = F::cast_from(0.86748650402413918736e-1_f64) * t48313;
    let t92022 = F::cast_from(4.0_f64) * t85986;
    let t92023 = -t47096 - t47098 + t92019 - t92020 - t92021 + t40067 - t40072 - t47109 + t92022 + t47116 - t47118 + t47122;
    (t92019, t92020, t92021, t92022, t92023)
}
