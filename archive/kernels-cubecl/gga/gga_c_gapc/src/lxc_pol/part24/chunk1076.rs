//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1076/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1076<F: Float>(t11775: F, t28254: F, t11990: F, t2817: F, t11997: F, t2639: F, t188: F, t1903: F, t190: F, t2660: F, t286: F, t442: F, t8139: F) -> (F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t33228 = t11775 * t28254;
    let t33230 = t11990 * t2817;
    let t33232 = t11997 * t2639;
    let t33235 = t188 * t1903 * pi;
    let t33240 = t2660 * t33235 * t8139 * t190 * t286 * t442;
    (t33228, t33230, t33232, t33235, t33240)
}
