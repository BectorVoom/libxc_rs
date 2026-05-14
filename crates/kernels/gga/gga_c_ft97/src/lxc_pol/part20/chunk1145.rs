//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1145/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1145<F: Float>(t107966: F, t108008: F, t110010: F, t110019: F, t110024: F, t110254: F, t13885: F, t14127: F, t14140: F, t14163: F, t1901: F, t193: F, t241: F, t242: F, t24599: F, t24668: F, t2579: F, t258: F, t2606: F, t3837: F, t3859: F, t3864: F, t3887: F, t42385: F, t446: F, t684: F, t6852: F, t6917: F, t89: F, t97451: F, t97463: F, t97470: F, t97472: F, t97701: F, t97810: F) -> (F,) {
    let t110272 = 4.0 * t1901 * t110010 * t6852 * t2579 - 4.0 / 3.0 * t1901 * t13885 * t97810 * t3859 - 4.0 / 3.0 * t1901 * t14127 * t110019 * t3864 + 2.0 / 9.0 * t1901 * t2606 * t110024 * t684 + 4.0 / 3.0 * t1901 * t13885 * t24668 * t14140 + 2.0 / 9.0 * t97451 + 2.0 / 9.0 * t97463 - 2.0 / 9.0 * t1901 * t14163 * t108008 + t1901 * t42385 * t6917 / 9.0 + t89 * t193 * t241 * t110254 * t258 / 3.0 + 4.0 / 3.0 * t446 * t242 * t107966 - 8.0 / 27.0 * t97470 + 4.0 / 9.0 * t1901 * t97701 * t3887 - 4.0 / 3.0 * t1901 * t13885 * t24599 * t3837 - 2.0 / 9.0 * t97472;
    (t110272,)
}
