//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1040/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1040(t31081: f64, t3564: f64, t1428: f64, t30605: f64, t457: f64, t12872: f64, t30892: f64, t1421: f64, t26710: f64, t26712: f64, t30738: f64, t31063: f64, t31067: f64, t31071: f64, t31075: f64, t31078: f64, t338: f64, t456: f64) -> (f64, f64, f64) {
    let t31082 = t3564 * t31081;
    let t31089 = t1428 * t30605;
    let t31090 = t457 * t31089;
    let t31093 = t12872 * t30892;
    let t31094 = t457 * t31093;
    let t31097 = 0.295669335e-2_f64 * t1421 * t31063 + 0.295669335e-2_f64 * t1421 * t31067 - 0.19711289e-2_f64 * t1421 * t31071 - 0.19711289e-2_f64 * t1421 * t31075 - 0.39422577999999999999e-2_f64 * t1421 * t31078 + 0.887008005e-2_f64 * t1421 * t31082 + 0.39422577999999999999e-2_f64 * t26710 + 0.295669335e-2_f64 * t26712 - 4.0_f64 * t338 * t30738 + 0.1478346675e-2_f64 * t456 * t31090 - 0.59133867e-2_f64 * t456 * t31094;
    (t31089, t31093, t31097)
}
