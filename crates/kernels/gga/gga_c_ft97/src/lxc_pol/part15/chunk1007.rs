//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1007/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1007<F: Float>(t5064: F, t41409: F, t13927: F, t21499: F, t10052: F, t5147: F, t1091: F, t14159: F, t18729: F, t1901: F, t21472: F, t21490: F, t21664: F, t242: F, t2599: F, t3977: F, t42334: F, t446: F, t4965: F, t4969: F, t4973: F, t5181: F, t724: F, t729: F, t81545: F, t81547: F, t9803: F) -> (F, F, F, F) {
    let t89441 = t5064 * t5064;
    let t89442 = t41409 * t89441;
    let t89456 = t13927 * t21499;
    let t89465 = t10052 * t5064 * t5147;
    let t89472 = 2.0 / 3.0 * t1901 * t2599 * t18729 * t4973 + 4.0 / 9.0 * t1901 * t9803 * t18729 * t4965 + 8.0 * t446 * t242 * t89442 + 4.0 / 3.0 * t446 * t724 * t5181 * t4969 - 4.0 / 9.0 * t81545 - 4.0 / 9.0 * t81547 + 8.0 / 3.0 * t1901 * t42334 * t21472 * t1091 + 8.0 * t446 * t242 * t89456 + 4.0 * t446 * t729 * t3977 * t21490 - 12.0 * t446 * t242 * t89465 + 4.0 / 3.0 * t1901 * t14159 * t21664;
    (t89442, t89456, t89465, t89472)
}
