//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1258/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1258<F: Float>(t2206: F, t4170: F, t6786: F, t22530: F, t4166: F, t6802: F, t10984: F, t2211: F, t3341: F, t8966: F, t10995: F, t2205: F, t790: F, t10989: F, t30832: F, t30835: F, t30837: F, t30840: F) -> (F, F, F, F, F, F, F, F) {
    let t30843 = t6786 * t4170 * t2206;
    let t30846 = t22530 * t4166 * t2206;
    let t30849 = t6802 * t4170 * t2206;
    let t30851 = t10984 * t2211;
    let t30853 = t3341 * t8966;
    let t30856 = t2205 * t10995 * t790;
    let t30858 = t10989 * t2211;
    let t30860 = 0.3071625e0 * t30832 + 0.3071625e0 * t30835 + 0.15358125e0 * t30837 - 0.3560484375e1 * t30840 + 0.142419375e1 * t30843 + 0.1151859375e0 * t30846 - 0.76790625e-1 * t30849 + 0.142419375e1 * t30851 - 0.1898925e1 * t30853 - 0.1898925e1 * t30856 - 0.9494625e0 * t30858;
    (t30843, t30846, t30849, t30851, t30853, t30856, t30858, t30860)
}
