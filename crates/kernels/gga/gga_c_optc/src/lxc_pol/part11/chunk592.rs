//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 592/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk592<F: Float>(t2672: F, t4941: F, t1: F, t313: F, t297: F, t312: F, t4961: F, t894: F, t123: F, t323: F, t287: F, t914: F, t1325: F, t3927: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4996 = t4941 * t2672;
    let t4997 = t4996 * t1;
    let t4998 = t313 * t4997;
    let t5002 = t4941 * t1 * t297;
    let t5003 = t313 * t5002;
    let t5006 = t312 * t4961;
    let t5007 = t5006 * t297;
    let t5008 = t894 * t5007;
    let t5011 = t4996 * t123;
    let t5012 = t323 * t5011;
    let t5016 = t4941 * t123 * t297;
    let t5017 = t323 * t5016;
    let t5021 = t287 * t4961 * t297;
    let t5022 = t914 * t5021;
    let t5025 = t3927 * t1325;
    (t4997, t4998, t5002, t5003, t5007, t5008, t5011, t5012, t5016, t5017, t5021, t5022, t5025)
}
