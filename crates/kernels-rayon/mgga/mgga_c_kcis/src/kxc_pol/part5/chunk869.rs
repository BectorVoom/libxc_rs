//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 869/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk869(t1409: f64, t7141: f64, t4023: f64, t4047: f64, t4050: f64, t4053: f64, t4054: f64, t4060: f64, t5814: f64, t5816: f64, t5829: f64, t5831: f64, t5833: f64, t6281: f64) -> (f64, f64) {
    let t7142 = t1409 * t7141;
    let t7155 = t4047 - t4050 - t4053 - 0.23911438650126355246e-1_f64 * t5814 + 0.20718155631185227504e-3_f64 * t5816 - t4054 + t4060 - 0.23526125e-4_f64 * t5829 + 0.9368e-2_f64 * t5831 - 0.26416666666666666666e-2_f64 * t5833 - 0.23911438650126355246e-1_f64 * t4023 * t6281;
    (t7142, t7155)
}
