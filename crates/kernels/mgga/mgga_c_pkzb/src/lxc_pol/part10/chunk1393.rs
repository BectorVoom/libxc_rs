//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1393/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1393<F: Float>(t3147: F, t8289: F, t10183: F, t2328: F, t2295: F, t891: F, t898: F, t9929: F, t2313: F, t3840: F, t3819: F, t6121: F, t2321: F, t8293: F, t10170: F, t27850: F, t27905: F, t27908: F, t27911: F, t27960: F, t27963: F) -> (F, F, F, F, F, F, F, F) {
    let t27965 = 0.20508037716432813315e4 * t3147 * t8289;
    let t27967 = 0.34631718211362927518e2 * t2328 * t10183;
    let t27971 = 0.23392894490538584828e1 * t898 * t2295 * t9929 * t891;
    let t27974 = 0.35089341735807877242e1 * t898 * t3840 * t2313;
    let t27975 = t6121 * t3819;
    let t27978 = 0.10389515463408878255e3 * t898 * t27975 * t2321;
    let t27980 = 0.34631718211362927517e2 * t3147 * t8293;
    let t27982 = 0.20508037716432813316e4 * t2328 * t10170;
    let t27983 = -t27960 - t27963 - t27965 - t27967 + t27971 - t27974 + t27978 - t27980 + t27850 + t27905 - t27982 + t27908 + t27911;
    (t27965, t27967, t27971, t27974, t27978, t27980, t27982, t27983)
}
