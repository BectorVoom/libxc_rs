//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1191/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1191<F: Float>(t109755: F, t1168: F, t18471: F, t6135: F, t18712: F, t24531: F, t31029: F, t684: F, t10007: F, t109844: F, t109863: F, t109875: F, t110438: F, t111283: F, t111518: F, t11593: F, t14163: F, t14182: F, t14187: F, t14196: F, t18438: F, t18442: F, t18497: F, t18514: F, t18707: F, t1901: F, t2347: F, t2360: F, t28386: F, t30986: F, t3886: F, t52018: F, t6940: F) -> (F, F, F, F, F) {
    let t121882 = t109755 * t1168;
    let t121889 = t6135 * t18471;
    let t121897 = t24531 * t18712;
    let t121914 = t31029 * t684;
    let t121928 = t109844 + 8.0 / 9.0 * t11593 * t14182 * t28386 * t18497 - 2.0 / 9.0 * t1901 * t14163 * t121889 - t1901 * t10007 * t6135 * t18707 / 9.0 - 2.0 / 9.0 * t1901 * t14196 * t121897 - 4.0 / 9.0 * t1901 * t14182 * t6940 * t2360 * t3886 + 4.0 / 27.0 * t1901 * t14187 * t6940 * t2347 * t3886 - 4.0 / 9.0 * t1901 * t52018 * t30986 + t109863 - 2.0 / 9.0 * t1901 * t14163 * t121914 - 4.0 / 9.0 * t1901 * t111518 * t18438 + 4.0 / 27.0 * t1901 * t110438 * t18442 + t109875 - 4.0 / 9.0 * t1901 * t14187 * t111283 * t18514;
    (t121882, t121889, t121897, t121914, t121928)
}
