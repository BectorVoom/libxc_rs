//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1206/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1206(t102917: f64, t102922: f64, t107044: f64, t1375: f64, t1843: f64, t20609: f64, t2092: f64, t27009: f64, t27068: f64, t29361: f64, t3887: f64, t5321: f64, t6440: f64, t6460: f64, t6461: f64, t7194: f64, t74849: f64, t7936: f64, t90659: f64, t90663: f64) -> f64 {
    let t107716 = 6.0_f64 * t1375 * t3887 * t7936 * t6460 - 3.0_f64 * t102922 * t1843 + 6.0_f64 * t27009 * t6440 - t74849 * t2092 - 3.0_f64 * t27068 * t6461 - 0.38381794893125283518e0_f64 * t90659 - 6.0_f64 * t7194 * t20609 - 0.49348022005446793095e-1_f64 * t90663 - 3.0_f64 * t5321 * t29361 - 0.16449340668482264365e-1_f64 * t107044 - 6.0_f64 * t102917 * t1843;
    t107716
}
