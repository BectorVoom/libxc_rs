//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1149/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1149<F: Float>(t6863: F, t8232: F, t2486: F, t6154: F, t1882: F, t28252: F, t10085: F, t107797: F, t109653: F, t13885: F, t13922: F, t14171: F, t14188: F, t1901: F, t2405: F, t2413: F, t242: F, t2459: F, t24737: F, t2574: F, t2599: F, t27836: F, t28123: F, t28153: F, t42996: F, t446: F, t6848: F, t6940: F, t729: F, t762: F, t773: F, t97629: F, t97777: F, t9803: F) -> (F,) {
    let t110420 = t8232 * t6863;
    let t110438 = t2486 * t6154;
    let t110447 = 2.0 / 9.0 * t1882 * t28252;
    let t110457 = t1901 * t2599 * t28123 * t2413 / 9.0 + 2.0 / 27.0 * t1901 * t9803 * t28123 * t2405 + t97629 + 2.0 / 3.0 * t446 * t242 * t107797 + 4.0 / 27.0 * t110420 + t1901 * t42996 * t6848 / 9.0 + 2.0 / 9.0 * t1901 * t10085 * t28153 - 2.0 / 3.0 * t1901 * t13885 * t24737 * t13922 + 2.0 / 3.0 * t446 * t242 * t109653 - 2.0 / 9.0 * t1901 * t97777 * t14171 + 4.0 / 27.0 * t1901 * t110438 * t14188 + t446 * t729 * t6154 * t13922 / 3.0 - t110447 + t446 * t729 * t762 * t6940 * t2459 / 3.0 + 4.0 / 3.0 * t446 * t2574 * t773 * t27836;
    (t110457,)
}
