//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1278/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1278<F: Float>(t30832: F, t30835: F, t30837: F, t30840: F, t30843: F, t30846: F, t30849: F, t30851: F, t30853: F, t30856: F, t30858: F, t26298: F, t26301: F, t26304: F, t30747: F, t30750: F, t30778: F, t30861: F, t30867: F, t30871: F, t30877: F, t30879: F, t30882: F) -> (F, F) {
    let t31332 = 0.16504875e0 * t30832 + 0.16504875e0 * t30835 + 0.82524375e-1 * t30837 - 0.485484375e1 * t30840 + 0.19419375e1 * t30843 + 0.6189328125e-1 * t30846 - 0.412621875e-1 * t30849 + 0.19419375e1 * t30851 - 0.258925e1 * t30853 - 0.258925e1 * t30856 - 0.1294625e1 * t30858;
    let t31345 = -0.412621875e-1 * t30861 - 0.18786444444444444444e1 * t26298 + 0.16102666666666666667e1 * t26301 - 0.60385e0 * t26304 + 0.27595e0 * t30867 + 0.49671e0 * t30871 + 0.40256666666666666667e0 * t30747 - 0.60385e0 * t30750 + 0.905775e0 * t30778 + 0.776775e1 * t30877 - 0.16504875e0 * t30879 - 0.258925e1 * t30882;
    (t31332, t31345)
}
