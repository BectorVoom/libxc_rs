//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1045/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1045<F: Float>(t1882: F, t25222: F, t25362: F, t681: F, t89: F, t25320: F, t25190: F, t25185: F, t10696: F, t1495: F, t25255: F, t24918: F, t25259: F, t25183: F, t870: F, t25480: F, t6213: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t99887 = t1882 * t25222;
    let t99895 = t89 * t681 * t25362;
    let t99909 = t1882 * t25320;
    let t99911 = t1882 * t25190;
    let t99916 = t1882 * t25185;
    let t99918 = t1495 * t10696;
    let t99923 = t1882 * t25255;
    let t99925 = t1882 * t24918;
    let t99938 = t1882 * t25259;
    let t99948 = t870 * t25183;
    let t99975 = t25480 * t6213;
    (t99887, t99895, t99909, t99911, t99916, t99918, t99923, t99925, t99938, t99948, t99975)
}
