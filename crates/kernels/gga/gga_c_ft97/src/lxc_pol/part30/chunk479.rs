//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 479/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk479<F: Float>(t317: F, t7612: F, t193: F, t1477: F, t1506: F, t2862: F, t319: F, t7584: F, t1476: F, t1508: F, t840: F, t1501: F) -> (F, F, F, F, F, F, F) {
    let t7613 = t7612 * t317;
    let t7614 = t193 * t7613;
    let t7617 = t1477 * t1506;
    let t7618 = t193 * t7617;
    let t7622 = t2862 * t319 * t7584;
    let t7626 = t840 * t1508 * t1476;
    let t7629 = t1476 * t1501;
    (t7613, t7614, t7617, t7618, t7622, t7626, t7629)
}
