//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1186/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1186<F: Float>(t27929: F, t6745: F, t107793: F, t107806: F, t107809: F, t18524: F, t18724: F, t24204: F, t24231: F, t24232: F, t27958: F, t27965: F, t28010: F, t30867: F, t30879: F, t30906: F, t30924: F, t5165: F, t5996: F, t6002: F, t96339: F, t96798: F) -> (F,) {
    let t121732 = t6745 * t27929;
    let t121736 = 2.0 / 9.0 * t6002 * t96798 * t30924 + 2.0 / 9.0 * t6002 * t24231 * t96339 * t5165 + 2.0 / 9.0 * t6002 * t24231 * t24232 * t18724 - 4.0 / 9.0 * t28010 * t24231 * t24232 * t18524 + t107793 + t107806 - t107809 - t24204 * t30879 / 9.0 - 2.0 / 3.0 * t6745 * t27965 - 2.0 / 3.0 * t6745 * t27958 + t5996 * t30867 / 6.0 - t121732 / 9.0 - 2.0 / 3.0 * t5996 * t30906;
    (t121736,)
}
