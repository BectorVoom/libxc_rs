//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1821/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1821(t74130: f64, t74132: f64, t48313: f64, t85986: f64, t40067: f64, t40072: f64, t47096: f64, t47098: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64) -> (f64, f64, f64, f64, f64) {
    let t92019 = 0.70178683471615754484e1_f64 * t74130;
    let t92020 = 48.0_f64 * t74132;
    let t92021 = 0.86748650402413918736e-1_f64 * t48313;
    let t92022 = 4.0_f64 * t85986;
    let t92023 = -t47096 - t47098 + t92019 - t92020 - t92021 + t40067 - t40072 - t47109 + t92022 + t47116 - t47118 + t47122;
    (t92019, t92020, t92021, t92022, t92023)
}
