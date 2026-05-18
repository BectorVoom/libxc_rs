//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 389/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk389<F: Float>(t1970: F, t2102: F, t1792: F, t582: F, t1796: F, t1984: F, t2: F, t1986: F, t24: F, t2075: F, t586: F, t2092: F, t2093: F, t2095: F, t2098: F, t462: F, t92: F) -> (F, F, F, F, F, F, F) {
    let t2103 = t2102 * t1970;
    let t2106 = t582 * t1792;
    let t2109 = t582 * t1796;
    let t2112 = t1984 * t2;
    let t2114 = t24 * t2112 * t1986;
    let t2118 = t24 * t586 * t2075;
    let t2120 = t2092 + F::new(2.0) / F::new(9.0) * t2093 + F::new(2.0) / F::new(3.0) * t2095 - F::new(2.0) / F::new(9.0) * t462 * t2098 + F::new(2.0) / F::new(3.0) * t462 * t2103 + F::new(2.0) / F::new(3.0) * t462 * t2106 - t462 * t2109 / F::new(3.0) + F::new(2.0) * t92 * t2114 - t92 * t2118;
    (t2103, t2106, t2109, t2112, t2114, t2118, t2120)
}
