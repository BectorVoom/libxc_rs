//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1029/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1029<F: Float>(t31146: F, t31210: F, t31271: F, t31318: F, t1137: F, t1403: F, t1454: F, t247: F, t27969: F, t30921: F, t30925: F, t30931: F, t30934: F, t30936: F, t30939: F, t30942: F, t30946: F, t30948: F, t30950: F, t30954: F, t31061: F, t31064: F, t31098: F, t4915: F, t5059: F, t6002: F, t6745: F, t6754: F, t6945: F) -> (F, F) {
    let t31320 = t31146 + t31210 + t31271 + t31318;
    let t31322 = t1403 * t30921 + 2.0 / 9.0 * t27969 + 2.0 / 9.0 * t6002 * t30925 - 2.0 / 3.0 * t6745 * t6754 + 4.0 * t30931 + 8.0 * t30934 + 8.0 * t30936 + t6002 * t30939 / 9.0 + 4.0 * t30942 - 2.0 * t1137 * t6945 - 4.0 * t30946 - 2.0 * t30948 - 2.0 * t30950 - t5059 * t1454 - t4915 * t1454 - 4.0 * t30954 - 2.0 * t31061 - 12.0 * t31064 + 2.0 * t31098 - t247 * t31320;
    (t31320, t31322)
}
