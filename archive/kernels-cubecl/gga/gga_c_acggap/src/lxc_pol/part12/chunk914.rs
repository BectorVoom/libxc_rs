//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 914/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk914<F: Float>(t1988: F, t7792: F, t7796: F, t7799: F, t1078: F, t1980: F, t1982: F, t1983: F, t1997: F, t3036: F, t3213: F, t1035: F, t1039: F, t7613: F) -> (F, F, F, F, F) {
    let t30891 = t1988 * t7792;
    let t30893 = t7799 * t7796;
    let t30901 = t1980 * t1982 * t1078 * t1983;
    let t30904 = t3036 * t1997 * t3213;
    let t30907 = t1035 * t7613 * t1039;
    (t30891, t30893, t30901, t30904, t30907)
}
