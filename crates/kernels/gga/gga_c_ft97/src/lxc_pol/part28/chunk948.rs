//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 948/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk948<F: Float>(t137564: F, t3219: F, t34565: F, t8466: F, t1882: F, t34773: F, t34770: F, t34568: F, t34737: F, t1339: F, t137908: F, t137921: F, t137923: F, t137980: F, t137987: F, t137997: F, t144645: F, t144745: F, t145761: F, t1871: F, t1901: F, t22940: F, t25990: F, t3214: F, t34562: F, t34632: F, t379: F, t39107: F, t446: F, t452: F, t47548: F, t60426: F, t6478: F, t7229: F, t83: F) -> (F, F, F, F) {
    let t146552 = t137564 * t3219;
    let t146561 = t8466 * t34565;
    let t146585 = t1882 * t34773;
    let t146587 = t1882 * t34770;
    let t146593 = t8466 * t34568;
    let t146598 = t1882 * t34737;
    let t146601 = 4.0 / 3.0 * t446 * t1871 * t1339 * t25990 + 2.0 / 3.0 * t446 * t83 * t146552 - 2.0 / 9.0 * t137908 - t446 * t83 * t144745 / 3.0 - 2.0 / 9.0 * t137921 + 4.0 / 3.0 * t446 * t83 * t146561 + t137923 / 9.0 + 2.0 / 3.0 * t446 * t452 * t22940 * t6478 - 2.0 / 3.0 * t446 * t83 * t145761 + 8.0 / 3.0 * t1901 * t60426 * t7229 * t3214 + 2.0 / 3.0 * t1901 * t47548 * t34562 * t379 + 2.0 / 9.0 * t1901 * t39107 * t34632 * t379 - 2.0 / 9.0 * t146585 - t146587 / 9.0 - t446 * t83 * t144645 / 3.0 + t137980 / 9.0 + 2.0 / 3.0 * t446 * t83 * t146593 + t137987 / 9.0 - 2.0 / 9.0 * t146598 - 4.0 / 9.0 * t137997;
    (t146552, t146561, t146593, t146601)
}
