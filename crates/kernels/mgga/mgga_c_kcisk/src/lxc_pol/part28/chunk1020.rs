//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1020/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1020<F: Float>(t1709: F, t23544: F, t7107: F, t7115: F, t4881: F, t8708: F, t10937: F, t11091: F, t11092: F, t17399: F, t17426: F, t17594: F, t17602: F, t23463: F, t23469: F, t23475: F, t23478: F) -> (F, F, F, F) {
    let t23545 = t23544 * t1709;
    let t23547 = t7115 * t7107;
    let t23549 = t4881 * t8708;
    let t23550 = t23549 * t1709;
    let t23565 = -0.13418888888888888889e0 * t10937 - t11091 - t11092 - 0.40256666666666666668e0 * t17399 + t17594 - t17602 + 0.73586666666666666667e-1 * t17426 - 0.33547222222222222222e0 * t23463 + 0.80513333333333333332e0 * t23469 - 0.181155e1 * t23475 - 0.24154e1 * t23478;
    (t23545, t23547, t23550, t23565)
}
