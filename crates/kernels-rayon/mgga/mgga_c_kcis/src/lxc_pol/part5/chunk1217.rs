//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1217/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1217(t1851: f64, t829: f64, t4580: f64, t3515: f64, t4566: f64, t11020: f64, t1233: f64, t18866: f64, t18868: f64, t18870: f64, t18872: f64, t18874: f64, t18947: f64, t18949: f64, t18970: f64, t18973: f64, t18976: f64, t18980: f64, t18983: f64, t18987: f64, t18993: f64, t19044: f64, t4741: f64, t4760: f64, t5261: f64) -> (f64, f64, f64) {
    let t20344 = t1851 * t829;
    let t20345 = t4580 * t20344;
    let t20346 = t3515 * t20345;
    let t20349 = t4566 * t20344;
    let t20350 = t11020 * t20349;
    let t20361 = t18866 + t18868 + t18870 - t18872 + t18874 + t18947 + t18949 - t18970 - t18973 - t18976 + t18980 + t18983 + t18987 + 0.11696446794910408142e1_f64 * t1233 * t19044 + 0.23392893589820816284e1_f64 * t5261 * t4741 - 0.11696446794910408142e1_f64 * t5261 * t4760 - t18993;
    (t20346, t20350, t20361)
}
