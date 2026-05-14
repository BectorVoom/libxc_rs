//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 876/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk876<F: Float>(t33577: F, t7732: F, t8461: F, t1843: F, t8460: F, t651: F, t5542: F, t8595: F, t2014: F, t1868: F, t4147: F, t32119: F, t13272: F, t8435: F, t1497: F, t8441: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33578 = 2.0 * t33577;
    let t33579 = t7732 * t8461;
    let t33580 = 2.0 * t33579;
    let t33581 = t1843 * t8460;
    let t33582 = t651 * t33581;
    let t33583 = 2.0 * t33582;
    let t33594 = t8595 * t5542;
    let t33595 = t2014 * t33594;
    let t33596 = t4147 * t1868;
    let t33597 = t32119 * t33596;
    let t33599 = 3.0 * t2014 * t33597;
    let t33609 = t13272 * t8435;
    let t33612 = t8441 * t1497;
    (t33578, t33580, t33581, t33583, t33594, t33595, t33597, t33599, t33609, t33612)
}
