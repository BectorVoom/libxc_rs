//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1401/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1401<F: Float>(t1632: F, t551: F, t574: F, t9948: F, t10137: F, t1600: F, t10090: F, t2196: F, t1592: F, t20623: F, t2526: F, t25501: F, t25505: F, t25521: F, t29810: F, t29814: F, t29822: F, t29839: F, t29842: F, t3216: F, t552: F, t910: F, t9365: F) -> (F,) {
    let t33972 = t574 * t551 * t1632 * t9948;
    let t33976 = t1600 * t10137;
    let t33980 = t2196 * t551 * t1632 * t10090;
    let t33982 = 0.39006997830244208535e0 * t1592 * t551 * t552 * t9365 * t910 + 0.39006997830244208535e0 * t1592 * t551 * t552 * t3216 * t2526 + 0.92480845007273388189e0 * t20623 + 0.12805040077930161442e1 * t29810 + 0.20803732176130244552e1 * t29814 + 0.38087975358139160776e-1 * t25501 + 0.57131963037208741164e-1 * t25505 + 0.20803732176130244552e1 * t29822 + 0.11557628986739024751e0 * t33972 + 0.32927245914677557992e-1 * t29839 - 0.32927245914677557992e-1 * t29842 - t25521 - 0.38415120233790484324e0 * t33976 - 0.41607464352260489104e1 * t33980;
    (t33982,)
}
