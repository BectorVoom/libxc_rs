//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1302/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1302<F: Float>(t109822: F, t122552: F, t124361: F, t124926: F, t1403: F, t1454: F, t17718: F, t18178: F, t18459: F, t18497: F, t18514: F, t193: F, t24231: F, t27976: F, t28010: F, t28015: F, t28030: F, t28031: F, t28037: F, t28043: F, t30896: F, t4915: F, t5179: F, t6002: F, t6062: F, t6064: F, t6192: F, t6745: F) -> (F,) {
    let t125391 = t30896 * t6064 / 6.0 - t109822 - 2.0 * t122552 - 2.0 * t124926 - 2.0 / 9.0 * t6002 * t24231 * t28031 * t18459 - t6002 * t28030 * t28037 * t18514 / 3.0 - 4.0 / 9.0 * t28010 * t28030 * t28031 * t18497 + 2.0 / 9.0 * t28015 * t28043 - t4915 * t6192 - t18178 * t1454 - 4.0 * t124361 - t17718 * t1454 - 2.0 / 3.0 * t6745 * t27976 + t1403 * t193 * t6062 * t5179 / 6.0;
    (t125391,)
}
