//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 828/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk828<F: Float>(t2134: F, t27: F, t4895: F, t649: F, t6355: F, t7810: F, t2344: F, t35674: F, t36391: F, t9222: F, t35551: F, t1679: F, t7900: F, t5016: F, t8404: F, t4601: F, t8407: F) -> (F, F, F, F, F, F, F, F) {
    let t40607 = t2134 * t27 * t649 * t4895;
    let t40610 = t6355 * t7810;
    let t40614 = t35674 * t2344;
    let t40619 = t9222 * t36391;
    let t40621 = t9222 * t35551;
    let t40623 = t1679 * t7900;
    let t40625 = t5016 * t8404;
    let t40627 = t4601 * t8407;
    (t40607, t40610, t40614, t40619, t40621, t40623, t40625, t40627)
}
