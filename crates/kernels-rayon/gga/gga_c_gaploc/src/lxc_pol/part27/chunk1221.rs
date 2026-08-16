//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1221/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1221(t32634: f64, t10640: f64, t7137: f64, t22775: f64, t2508: f64, t9014: f64, t1949: f64, t3444: f64, t10743: f64, t731: f64, t22044: f64, t2580: f64, t2958: f64, t5269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32635 = 0.99692006687028833308e-3_f64 * t32634;
    let t32639 = 0.12304841968227558854e0_f64 * t7137 * t10640;
    let t32642 = 0.92286314761706691403e-1_f64 * t2508 * t9014 * t22775;
    let t32643 = t1949 * t3444;
    let t32644 = 0.85450291446024714264e-3_f64 * t32643;
    let t32645 = t731 * t10743;
    let t32646 = 0.85450291446024714264e-3_f64 * t32645;
    let t32650 = 0.30762104920568897134e-1_f64 * t5269 * t2580 * t2958 * t22044;
    (t32635, t32639, t32642, t32644, t32646, t32650)
}
