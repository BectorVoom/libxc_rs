//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1279/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1279<F: Float>(t29134: F, t8392: F, t1882: F, t29401: F, t14602: F, t99918: F, t10666: F, t112399: F, t15195: F, t15369: F, t1901: F, t2405: F, t2413: F, t24894: F, t25253: F, t2749: F, t28524: F, t2857: F, t29302: F, t296: F, t4167: F, t4176: F, t446: F, t57186: F, t7045: F, t7131: F, t835: F, t840: F, t98899: F, t99670: F, t99676: F, t99678: F) -> (F, F) {
    let t114626 = 2.0 / 27.0 * t8392 * t29134;
    let t114648 = 2.0 / 9.0 * t1882 * t29401;
    let t114665 = t99918 * t14602;
    let t114669 = -t114626 - 2.0 / 9.0 * t1901 * t15195 * t24894 - 4.0 / 3.0 * t1901 * t15369 * t98899 * t4167 + 4.0 / 27.0 * t1901 * t57186 * t28524 + 4.0 / 3.0 * t446 * t296 * t112399 - 2.0 / 9.0 * t99670 + 2.0 / 81.0 * t99676 - t99678 / 9.0 + 2.0 / 3.0 * t446 * t840 * t25253 * t4176 - t114648 - 2.0 / 27.0 * t446 * t2857 * t7131 * t2405 - t446 * t835 * t7131 * t2413 / 9.0 + 2.0 / 3.0 * t446 * t840 * t2749 * t29302 + t446 * t840 * t10666 * t7045 / 3.0 - 2.0 * t446 * t296 * t114665;
    (t114665, t114669)
}
