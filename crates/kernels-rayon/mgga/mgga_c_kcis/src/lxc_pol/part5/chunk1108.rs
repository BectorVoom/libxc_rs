//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1108/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1108(t1109: f64, t18685: f64, t345: f64, t10414: f64, t1102: f64, t14125: f64, t14127: f64, t14137: f64, t14168: f64, t14202: f64, t14204: f64, t14321: f64, t1697: f64, t18597: f64, t18601: f64, t18606: f64, t18608: f64, t18615: f64, t18620: f64, t18624: f64, t18627: f64, t18632: f64, t18636: f64, t18639: f64, t344: f64, t4768: f64, t6432: f64, t975: f64) -> (f64, f64) {
    let t18686 = t1109 * t18685;
    let t18687 = t345 * t18686;
    let t18690 = -0.1478346675e-2_f64 * t1102 * t18597 + 0.19711289e-2_f64 * t1102 * t18601 + t14125 - t14127 - 4.0_f64 * t975 * t6432 + 0.98556445e-3_f64 * t18606 + 0.13140859333333333333e-2_f64 * t18608 - 0.87605728888888888887e-3_f64 * t14137 + t14168 - 8.0_f64 * t1697 * t4768 - t14202 + t14204 - 0.19711289e-2_f64 * t10414 * t18615 + 0.26281718666666666666e-2_f64 * t10414 * t18620 + 0.26281718666666666666e-2_f64 * t10414 * t18624 - 0.21901432222222222222e-2_f64 * t14321 * t18627 + 0.98556445e-3_f64 * t10414 * t18632 - 0.19711289e-2_f64 * t10414 * t18636 - 0.39422578e-2_f64 * t10414 * t18639 + 0.1478346675e-2_f64 * t344 * t18687;
    (t18686, t18690)
}
