//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1183/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1183<F: Float>(t29915: F, t8392: F, t29951: F, t1882: F, t29992: F, t16246: F, t5743: F, t102599: F, t102614: F, t102626: F, t116105: F, t11906: F, t1825: F, t1871: F, t1901: F, t25846: F, t26245: F, t29831: F, t29961: F, t29995: F, t30012: F, t379: F, t4436: F, t446: F, t452: F, t47089: F, t488: F, t83: F, t8506: F, t8557: F, t979: F) -> (F, F) {
    let t116938 = t8392 * t29915;
    let t116944 = t8392 * t29951;
    let t116967 = t1882 * t29992;
    let t116973 = t16246 * t5743;
    let t116980 = -t116938 / 27.0 - 2.0 / 9.0 * t1901 * t8557 * t29831 * t379 - 2.0 / 27.0 * t116944 - 2.0 / 9.0 * t1901 * t8506 * t29995 + 2.0 / 9.0 * t1901 * t11906 * t26245 - 2.0 / 3.0 * t446 * t1871 * t488 * t5743 * t4436 + 16.0 / 27.0 * t102599 + 2.0 / 3.0 * t446 * t452 * t1825 * t29961 + 2.0 / 3.0 * t446 * t452 * t488 * t25846 * t979 + 2.0 / 3.0 * t116967 + 2.0 / 3.0 * t446 * t83 * t116105 + 8.0 / 27.0 * t102614 - t446 * t83 * t116973 / 3.0 - t102626 - 4.0 / 9.0 * t1901 * t47089 * t30012;
    (t116973, t116980)
}
