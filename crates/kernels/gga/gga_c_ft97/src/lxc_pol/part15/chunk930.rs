//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 930/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk930<F: Float>(t20655: F, t925: F, t1017: F, t20045: F, t11755: F, t12796: F, t17259: F, t1985: F, t2097: F, t2102: F, t40337: F, t40485: F, t462: F, t4714: F, t582: F, t78164: F, t78251: F, t86023: F, t86027: F, t86031: F, t86035: F, t86039: F, t86043: F, t86121: F, t86876: F, t9016: F, t9224: F) -> (F, F, F) {
    let t86902 = t925 * t20655;
    let t86906 = t20045 * t1017;
    let t86933 = -4.0 / 3.0 * t78251 + t40485 + 8.0 / 3.0 * t11755 * t12796 * t86876 + 8.0 * t462 * t582 * t86027 + 2.0 * t462 * t582 * t86039 + 4.0 / 3.0 * t462 * t2102 * t86902 + 4.0 / 3.0 * t462 * t2102 * t86906 - 80.0 / 81.0 * t462 * t40337 * t86121 - 36.0 * t462 * t9016 * t17259 * t4714 - t462 * t582 * t86031 / 3.0 + 8.0 * t462 * t1985 * t78164 * t1017 + 40.0 / 9.0 * t462 * t9224 * t86023 - 8.0 * t462 * t2097 * t86043 - 2.0 / 3.0 * t462 * t2097 * t86035;
    (t86902, t86906, t86933)
}
