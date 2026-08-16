//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2022/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2022<F: Float>(t22828: F, t80853: F, t80855: F, t22783: F, t3872: F, t1336: F, t2690: F, t6950: F, t1369: F, t22782: F, t3777: F, t3876: F) -> (F, F, F, F, F, F, F) {
    let t80857 = t80853 * t80855 * t22828;
    let t80859 = t22783 * t3872;
    let t80866 = t1336 * t6950 * t2690;
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    let t80870 = t80869 * t1369;
    let t80872 = t22783 * t3876;
    (t80857, t80859, t80866, t80867, t80869, t80870, t80872)
}
