//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1248/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1248<F: Float>(t11474: F, t8880: F, t3076: F, t34714: F, t11455: F, t1453: F, t505: F, t5526: F, t674: F, t34503: F, t9256: F, t26007: F, t3708: F, t9304: F) -> (F, F, F, F, F) {
    let t34832 = t11474 * t8880;
    let t34834 = t34714 * t3076;
    let t34839 = t11455 * t1453 * t505 * t674 * t5526;
    let t34846 = t34503 * t9256;
    let t34849 = t9304 * t3708 * t26007;
    (t34832, t34834, t34839, t34846, t34849)
}
