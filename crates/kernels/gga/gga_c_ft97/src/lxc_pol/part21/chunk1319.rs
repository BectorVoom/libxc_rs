//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1319/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1319<F: Float>(t1882: F, t30474: F, t106551: F, t106555: F, t107082: F, t11593: F, t119345: F, t120850: F, t120858: F, t120860: F, t121064: F, t13153: F, t143: F, t144: F, t160: F, t16964: F, t17026: F, t17066: F, t17071: F, t17076: F, t1901: F, t23443: F, t23571: F, t23581: F, t26995: F, t26999: F, t27414: F, t28: F, t3435: F, t4454: F, t446: F, t569: F, t5855: F, t89: F, t9115: F, t925: F) -> (F,) {
    let t121070 = t1882 * t30474;
    let t121072 = 2.0 / 3.0 * t446 * t144 * t119345 + 4.0 / 9.0 * t11593 * t13153 * t26995 + 2.0 / 27.0 * t1901 * t9115 * t23581 * t4454 - 2.0 / 27.0 * t120850 + t1901 * t23443 * t16964 / 9.0 - 4.0 / 3.0 * t1901 * t107082 * t17026 + t120858 / 27.0 - 2.0 / 27.0 * t120860 + 8.0 * t1901 * t106555 * t5855 * t17066 + 2.0 * t1901 * t26999 * t23571 * t17071 - 4.0 * t1901 * t26999 * t5855 * t17076 + 4.0 / 9.0 * t1901 * t106551 * t3435 - 2.0 / 9.0 * t446 * t569 * t27414 * t925 + t89 * t28 * t143 * t121064 * t160 / 3.0 - 2.0 / 9.0 * t121070;
    (t121072,)
}
