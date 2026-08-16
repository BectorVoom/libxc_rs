//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1043/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1043(t34286: f64, t34293: f64, t34315: f64, t34317: f64, t34392: f64, t34394: f64, t34396: f64, t34400: f64, t34433: f64, t34453: f64, t34468: f64, t34476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37009 = 0.90035438047946447644e-2_f64 * t34286;
    let t37012 = 0.32012600194825403606e-1_f64 * t34293;
    let t37021 = 0.17149607247227894789e-2_f64 * t34315;
    let t37022 = 0.25724410870841842184e-2_f64 * t34317;
    let t37066 = 0.26147916666666666667e0_f64 * t34392;
    let t37067 = 0.42874018118069736972e-3_f64 * t34394;
    let t37068 = 0.16006300097412701803e-1_f64 * t34396;
    let t37070 = 0.34299214494455789578e-2_f64 * t34400;
    let t37093 = 77.0_f64 / 864.0_f64 * t34433;
    let t37105 = 0.10718504529517434243e-2_f64 * t34453;
    let t37112 = 0.90035438047946447644e-2_f64 * t34468;
    let t37114 = 0.18868855373762491241e-2_f64 * t34476;
    (t37009, t37012, t37021, t37022, t37066, t37067, t37068, t37070, t37093, t37105, t37112, t37114)
}
