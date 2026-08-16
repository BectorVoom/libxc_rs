//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1128/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1128(t1881: f64, t8015: f64, t27741: f64, t12861: f64, t1607: f64, t4314: f64, t4455: f64, t779: f64, t9274: f64, t2531: f64, t2537: f64, t782: f64, t9266: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28891 = t1881 * t8015;
    let t28901 = 2.0_f64 * t27741;
    let t30409 = t1607 * t12861;
    let t30424 = t4455 * t4314;
    let t31271 = t779 * t9274;
    let t31274 = t2531 * t2537;
    let t35630 = t9266 * t782;
    (t28891, t28901, t30409, t30424, t31271, t31274, t35630)
}
