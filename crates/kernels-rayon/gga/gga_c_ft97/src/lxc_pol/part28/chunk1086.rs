//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1086/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1086(t137564: f64, t3219: f64, t34565: f64, t8466: f64, t1882: f64, t34773: f64, t34770: f64, t34568: f64, t34737: f64, t1339: f64, t137908: f64, t137921: f64, t137923: f64, t137980: f64, t137987: f64, t137997: f64, t144645: f64, t144745: f64, t145761: f64, t1871: f64, t1901: f64, t22940: f64, t25990: f64, t3214: f64, t34562: f64, t34632: f64, t379: f64, t39107: f64, t446: f64, t452: f64, t47548: f64, t60426: f64, t6478: f64, t7229: f64, t83: f64) -> (f64, f64, f64, f64) {
    let t146552 = t137564 * t3219;
    let t146561 = t8466 * t34565;
    let t146585 = t1882 * t34773;
    let t146587 = t1882 * t34770;
    let t146593 = t8466 * t34568;
    let t146598 = t1882 * t34737;
    let t146601 = 4.0_f64 / 3.0_f64 * t446 * t1871 * t1339 * t25990 + 2.0_f64 / 3.0_f64 * t446 * t83 * t146552 - 2.0_f64 / 9.0_f64 * t137908 - t446 * t83 * t144745 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t137921 + 4.0_f64 / 3.0_f64 * t446 * t83 * t146561 + t137923 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t452 * t22940 * t6478 - 2.0_f64 / 3.0_f64 * t446 * t83 * t145761 + 8.0_f64 / 3.0_f64 * t1901 * t60426 * t7229 * t3214 + 2.0_f64 / 3.0_f64 * t1901 * t47548 * t34562 * t379 + 2.0_f64 / 9.0_f64 * t1901 * t39107 * t34632 * t379 - 2.0_f64 / 9.0_f64 * t146585 - t146587 / 9.0_f64 - t446 * t83 * t144645 / 3.0_f64 + t137980 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t83 * t146593 + t137987 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t146598 - 4.0_f64 / 9.0_f64 * t137997;
    (t146552, t146561, t146593, t146601)
}
