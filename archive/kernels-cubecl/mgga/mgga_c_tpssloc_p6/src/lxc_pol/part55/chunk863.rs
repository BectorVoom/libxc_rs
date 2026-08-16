//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 863/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk863<F: Float>(t22779: F, t6937: F, t6950: F, t835: F, t1336: F, t1369: F, t3777: F, t6951: F, t6597: F, t6924: F, t281: F, t1307: F, t1361: F, t22690: F) -> (F, F, F, F, F, F) {
    let t22780 = t22779 * t6937;
    let t22782 = t6950 * t835;
    let t22783 = t1336 * t22782;
    let t22784 = t22783 * t1369;
    let t22788 = t3777 * t6951;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    let t22794 = t22690 * t1361 * t1307;
    (t22780, t22783, t22784, t22788, t22792, t22794)
}
