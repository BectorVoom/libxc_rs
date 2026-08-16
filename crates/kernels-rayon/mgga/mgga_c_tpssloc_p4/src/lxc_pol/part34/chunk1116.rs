//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1116/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1116(t81920: f64, t81954: f64, t2047: f64, t9971: f64, t81688: f64, t81716: f64, t82046: f64, t82122: f64, t82153: f64, t82218: f64, t1453: f64, t81439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84921 = 595.0_f64 / 2592.0_f64 * t81920;
    let t84932 = 0.67287926823567318088e-4_f64 * t81954;
    let t84953 = t9971 * t2047;
    let t84995 = 0.27415567780803773942e-2_f64 * t81688;
    let t85003 = 0.19739208802178717238e0_f64 * t81716;
    let t85027 = 0.55440370401180965083e0_f64 * t82046;
    let t85060 = 0.3244175520728446583e0_f64 * t82122;
    let t85101 = 0.27415567780803773942e-2_f64 * t82153;
    let t85129 = 0.55440370401180965083e0_f64 * t82218;
    let t86586 = t81439 * t1453;
    (t84921, t84932, t84953, t84995, t85003, t85027, t85060, t85101, t85129, t86586)
}
