//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1134/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1134<F: Float>(t28395: F, t8392: F, t28312: F, t13885: F, t13934: F, t14067: F, t14094: F, t14155: F, t14175: F, t14205: F, t14226: F, t18467: F, t1901: F, t2405: F, t24412: F, t24664: F, t24789: F, t24793: F, t27924: F, t3842: F, t3870: F, t42362: F, t446: F, t684: F, t6921: F, t729: F, t97422: F, t97424: F, t97701: F, t97810: F) -> (F,) {
    let t109968 = 2.0 / 27.0 * t8392 * t28395;
    let t109989 = 2.0 / 27.0 * t8392 * t28312;
    let t110007 = 2.0 / 27.0 * t1901 * t18467 * t24664 - t109968 + 2.0 / 9.0 * t1901 * t97701 * t3870 + 2.0 / 9.0 * t1901 * t24789 * t14155 + t1901 * t24789 * t14067 / 9.0 - 2.0 / 3.0 * t446 * t729 * t24412 * t14226 + 2.0 / 9.0 * t1901 * t24793 * t14094 + t1901 * t24793 * t13934 / 9.0 - t109989 - 2.0 / 9.0 * t1901 * t24793 * t14205 - 2.0 / 27.0 * t1901 * t42362 * t6921 * t2405 - 4.0 / 9.0 * t1901 * t14175 * t27924 * t684 - 4.0 / 3.0 * t1901 * t13885 * t97810 * t3842 - 2.0 / 9.0 * t97422 - t97424 / 27.0;
    (t110007,)
}
