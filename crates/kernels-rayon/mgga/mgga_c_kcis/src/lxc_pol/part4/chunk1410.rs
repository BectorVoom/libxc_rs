//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1410/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1410(t18292: f64, t18311: f64, t18331: f64, t18350: f64, t12940: f64, t1629: f64, t1636: f64, t17310: f64, t17314: f64, t17317: f64, t17325: f64, t17709: f64, t17710: f64, t17713: f64, t18266: f64, t18268: f64, t18271: f64, t4480: f64, t4481: f64, t633: f64) -> f64 {
    let t18352 = t18292 + t18311 + t18331 + t18350;
    let t18354 = -6.0_f64 * t12940 * t17713 - t1629 * t18352 - 2.0_f64 * t1636 * t17710 + t18266 * t633 + 2.0_f64 * t18268 * t4481 + 4.0_f64 * t18271 * t4480 + t17310 + t17314 - t17317 - t17325 + t17709;
    t18354
}
