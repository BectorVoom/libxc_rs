//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1087/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1087<F: Float>(t1841: F, t3487: F, t7275: F, t734: F, t10826: F, t2536: F, t1944: F, t3444: F, t10640: F, t7137: F, t22775: F, t2508: F, t9014: F, t1949: F, t10743: F, t731: F) -> (F, F, F, F, F, F, F) {
    let t32629 = 0.17090058289204942853e-2 * t1841 * t7275 * t3487 * t734;
    let t32633 = 0.17090058289204942853e-2 * t1841 * t2536 * t10826 * t734;
    let t32634 = t1944 * t3444;
    let t32635 = 0.99692006687028833308e-3 * t32634;
    let t32639 = 0.12304841968227558854e0 * t7137 * t10640;
    let t32642 = 0.92286314761706691403e-1 * t2508 * t9014 * t22775;
    let t32643 = t1949 * t3444;
    let t32644 = 0.85450291446024714264e-3 * t32643;
    let t32645 = t731 * t10743;
    (t32629, t32633, t32635, t32639, t32642, t32644, t32645)
}
