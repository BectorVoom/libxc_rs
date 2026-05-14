//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 806/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk806<F: Float>(t30080: F, t9948: F, t3928: F, t6449: F, t645: F, t6434: F, t5016: F, t9951: F, t9128: F, t1550: F, t2060: F, t30344: F, t45168: F, t45170: F, t45175: F, t45179: F, t45183: F, t45187: F, t45189: F, t45193: F, t45197: F, t45201: F, t45205: F, t45207: F) -> (F,) {
    let t45209 = t30080 * t9948;
    let t45212 = t3928 * t645 * t6449;
    let t45215 = t3928 * t645 * t6434;
    let t45217 = t5016 * t9951;
    let t45219 = t9128 * t9951;
    let t45222 = t1550 * t2060 * t30344;
    let t45224 = 0.13637330827122670864e-1 * t45168 + 0.85129199786595678796e-5 * t45170 - 0.1064114997332445985e-4 * t45175 - 0.51077519871957407276e-4 * t45179 + 0.76616279807936110914e-4 * t45183 + 0.25538759935978703638e-4 * t45187 + 0.85129199786595678796e-5 * t45189 + 0.85129199786595678796e-5 * t45193 + 0.31923449919973379548e-4 * t45197 - 0.25538759935978703639e-4 * t45201 - 0.85129199786595678796e-5 * t45205 - 0.68186654135613354322e-2 * t45207 + 0.17961362552795712846e0 * t45209 + 0.17961362552795712846e0 * t45212 + 0.17961362552795712846e0 * t45215 + 0.5987120850931904282e-1 * t45217 + 0.5987120850931904282e-1 * t45219 + 0.5987120850931904282e-1 * t45222;
    (t45224,)
}
