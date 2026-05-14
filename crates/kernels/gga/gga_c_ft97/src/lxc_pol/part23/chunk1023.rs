//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1023/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1023<F: Float>(t4969: F, t6074: F, t2599: F, t1424: F, t5147: F, t729: F, t762: F, t1456: F, t4973: F, t724: F, t2594: F, t4965: F, t1901: F, t28289: F, t31148: F, t31152: F, t31157: F, t31160: F, t31164: F, t31167: F, t31170: F, t31175: F, t31179: F, t31183: F, t31186: F, t31190: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t31193 = t6074 * t4969;
    let t31194 = t2599 * t31193;
    let t31197 = t1424 * t5147;
    let t31199 = t729 * t762 * t31197;
    let t31203 = t724 * t1456 * t4973;
    let t31207 = t2594 * t1456 * t4965;
    let t31210 = t1901 * t31148 / 9.0 + 2.0 / 3.0 * t446 * t31152 + 2.0 / 3.0 * t446 * t31157 + 2.0 / 9.0 * t1901 * t31160 + 2.0 / 9.0 * t1901 * t31164 + 2.0 / 9.0 * t1901 * t31167 + 2.0 / 9.0 * t1901 * t31170 + 2.0 / 27.0 * t28289 - 2.0 / 3.0 * t446 * t31175 + 2.0 / 27.0 * t1901 * t31179 - 4.0 / 3.0 * t1901 * t31183 + 2.0 / 9.0 * t1901 * t31186 + 2.0 / 9.0 * t1901 * t31190 - 2.0 / 9.0 * t1901 * t31194 + t446 * t31199 / 3.0 - t446 * t31203 / 9.0 - 2.0 / 27.0 * t446 * t31207;
    (t31193, t31194, t31197, t31199, t31203, t31207, t31210)
}
