//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1151/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1151<F: Float>(t1882: F, t28226: F, t1456: F, t9802: F, t258: F, t27742: F, t10007: F, t1091: F, t11593: F, t13706: F, t13892: F, t14104: F, t14127: F, t14175: F, t14254: F, t1901: F, t24569: F, t24738: F, t24789: F, t24852: F, t2599: F, t27983: F, t28128: F, t28276: F, t28298: F, t28301: F, t3972: F, t446: F, t52002: F, t6061: F, t6075: F, t65408: F, t684: F, t724: F, t729: F, t754: F, t762: F, t97681: F, t97683: F) -> (F,) {
    let t110517 = 2.0 / 27.0 * t1882 * t28226;
    let t110539 = t9802 * t1456;
    let t110543 = t258 * t27742;
    let t110557 = -2.0 / 3.0 * t1901 * t14127 * t28128 * t14254 + 2.0 / 9.0 * t1901 * t52002 * t6075 - 4.0 / 3.0 * t1901 * t65408 * t24738 + t110517 - 4.0 / 9.0 * t1901 * t14175 * t27983 * t684 + t1901 * t24789 * t14104 / 9.0 - 2.0 / 9.0 * t1901 * t10007 * t28276 * t684 + 4.0 / 9.0 * t11593 * t10007 * t24569 * t13892 - t97681 / 27.0 - 2.0 / 81.0 * t97683 - 4.0 * t1901 * t28298 * t754 * t28301 + 4.0 / 27.0 * t1901 * t110539 * t13706 + 2.0 / 9.0 * t1901 * t2599 * t110543 * t684 - t446 * t724 * t24852 * t1091 / 9.0 + 2.0 / 3.0 * t446 * t729 * t762 * t6061 * t3972;
    (t110557,)
}
