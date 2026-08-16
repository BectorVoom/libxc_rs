//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1049/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1049(t18463: f64, t18532: f64, t18584: f64, t18620: f64, t18663: f64, t18925: f64, t18957: f64, t18984: f64, t153: f64, t156: f64, t18054: f64, t18367: f64, t18369: f64, t18372: f64, t18375: f64, t18377: f64, t18379: f64, t18413: f64, t18415: f64, t18416: f64, t18419: f64, t18420: f64, t242: f64) -> (f64, f64) {
    let t18987 = t18463 + t18532 + t18584 + t18620 + t18663 + t18925 + t18957 + t18984;
    let t18991 = 0.10051538464260528225e1_f64 * t18367 + 0.10051538464260528225e1_f64 * t18369 + t18372 - 0.83762820535504401876e-1_f64 * t18054 * t242 - 0.33505128214201760751e0_f64 * t18375 - 0.50257692321302641126e0_f64 * t18377 - 0.33505128214201760751e0_f64 * t18379 - t18413 + t18415 - 0.10051538464260528225e1_f64 * t18416 - t18419 + 0.2010307692852105645e1_f64 * t18420 + 0.42708890021612718669e0_f64 * t153 * t156 * t18987;
    (t18987, t18991)
}
