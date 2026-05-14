//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1157/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1157<F: Float>(t28365: F, t8392: F, t1882: F, t28448: F, t28236: F, t28445: F, t109606: F, t109661: F, t14053: F, t14159: F, t14259: F, t1456: F, t1901: F, t2409: F, t242: F, t24578: F, t2574: F, t2579: F, t28023: F, t28098: F, t446: F, t6154: F, t6947: F, t713: F, t724: F, t729: F, t97809: F, t97815: F) -> (F,) {
    let t110803 = 4.0 / 9.0 * t8392 * t28365;
    let t110805 = 2.0 / 9.0 * t1882 * t28448;
    let t110817 = 4.0 / 9.0 * t1882 * t28236;
    let t110826 = 2.0 / 9.0 * t1882 * t28445;
    let t110840 = t110803 + t110805 + t446 * t729 * t6154 * t14259 / 3.0 - 2.0 / 9.0 * t1901 * t14159 * t24578 - t446 * t242 * t109606 / 3.0 + t97809 - t110817 + 4.0 / 3.0 * t446 * t2574 * t1456 * t14053 - 2.0 * t446 * t242 * t109661 - t110826 - 2.0 / 3.0 * t446 * t729 * t28098 * t713 - 8.0 / 27.0 * t97815 + 2.0 / 3.0 * t446 * t729 * t28023 * t2579 + 2.0 / 9.0 * t446 * t724 * t6947 * t2409;
    (t110840,)
}
