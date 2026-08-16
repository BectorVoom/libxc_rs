//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1785/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1785<F: Float>(t23030: F, t6643: F, t131: F, t244: F, t209: F, t1878: F, t2379: F, t6638: F, t6637: F, t6612: F, t835: F, t812: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23031 = t23030 * t6643;
    let t23032 = F::cast_from(0.26044789391763585244e-1_f64) * t23031;
    let t23033 = t244 * t131;
    let t23034 = t23033 * t209;
    let t23035 = t1878 * t23034;
    let t23036 = t6638 * t2379;
    let t23037 = t6637 * t23036;
    let t23038 = t23035 * t23037;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    (t23032, t23033, t23034, t23035, t23036, t23037, t23038, t23040, t23041)
}
