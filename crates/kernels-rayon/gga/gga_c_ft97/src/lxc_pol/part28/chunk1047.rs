//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1047/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1047(t1301: f64, t145335: f64, t32259: f64, t136720: f64, t136843: f64, t136968: f64, t145303: f64, t145312: f64, t145322: f64, t1608: f64, t2035: f64, t22736: f64, t22819: f64, t25779: f64, t25802: f64, t3099: f64, t32133: f64, t32228: f64, t34435: f64, t5551: f64, t58585: f64, t6441: f64, t7195: f64, t7318: f64, t7857: f64, t7867: f64, t92278: f64, t92377: f64) -> f64 {
    let t145337 = t32259 * t1301 * t145335;
    let t145339 = -0.19762785756235085044e-4_f64 * t7867 * t2035 * t7318 * t3099 + 0.11854761295685025975e-1_f64 * t32228 * t145303 - 0.90845139567911167717e-8_f64 * t1608 * t92377 * t5551 * t136968 * t6441 - 0.68116566383613497688e-3_f64 * t22819 * t7195 * t145312 + 0.45958162518691859408e-7_f64 * t22736 * t32133 * t25802 - 0.60102574844279699039e-6_f64 * t7857 * t58585 * t145322 + 0.22979081259345929704e-6_f64 * t92278 * t32133 * t25779 + 0.15322466011111111111e0_f64 * t32259 * t1301 * t145312 + 0.15322466011111111111e0_f64 * t136843 * t34435 + 0.15322466011111111111e0_f64 * t136720 * t34435 + 0.51074886703703703703e-1_f64 * t145337;
    t145339
}
