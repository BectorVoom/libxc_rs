//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1114/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1114<F: Float>(t10640: F, t7137: F, t22775: F, t2508: F, t9014: F, t1949: F, t3444: F, t10743: F, t731: F, t22044: F, t2580: F, t2958: F, t5269: F, t29194: F, t2936: F, t1890: F, t21446: F) -> (F, F, F, F, F, F, F) {
    let t32639 = 0.12304841968227558854e0 * t7137 * t10640;
    let t32642 = 0.92286314761706691403e-1 * t2508 * t9014 * t22775;
    let t32643 = t1949 * t3444;
    let t32644 = 0.85450291446024714264e-3 * t32643;
    let t32645 = t731 * t10743;
    let t32646 = 0.85450291446024714264e-3 * t32645;
    let t32650 = 0.30762104920568897134e-1 * t5269 * t2580 * t2958 * t22044;
    let t32653 = 0.10766736722199113997e0 * t2508 * t2936 * t29194;
    let t32657 = 0.1845726295234133828e0 * t2508 * t9014 * t1890 * t21446;
    (t32639, t32642, t32644, t32646, t32650, t32653, t32657)
}
