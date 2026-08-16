//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1682/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1682<F: Float>(t1298: F, t1300: F, t13190: F, t198: F, t336: F, t3801: F, t44096: F, t44100: F, t44103: F, t44106: F, t44108: F, t44111: F, t44114: F, t44122: F, t44123: F, t44126: F, t44984: F, t44987: F, t45448: F, t45494: F, t45544: F, t45895: F, t5023: F) -> F {
    let t45901 = t44096 + t44100 - t44103 + t44106 + t44108 - t44111 - t44114 - F::cast_from(4.0_f64) * t5023 * t13190 * t3801 * t1298 + t44122 - F::cast_from(6.0_f64) * t198 * t336 * t44123 * t44126 + t198 * t336 * (t45448 + t45494 + t45544 + t45895) * t1300 + t44984 - t44987;
    t45901
}
