//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1252/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1252<F: Float>(t1222: F, t1731: F, t1744: F, t1202: F, t1743: F, t225: F, t4940: F) -> (F, F, F, F) {
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4961 = t1202 * t1743;
    let t4964 = t4940 * t225;
    (t4957, t4959, t4961, t4964)
}
