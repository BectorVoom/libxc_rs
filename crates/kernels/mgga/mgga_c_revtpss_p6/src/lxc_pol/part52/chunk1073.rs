//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1073/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1073<F: Float>(t4147: F, t7535: F, t4248: F, t8461: F, t7732: F, t1843: F, t8460: F, t651: F, t1518: F, t1931: F) -> (F, F, F, F, F, F) {
    let t33183 = t4147 * t7535;
    let t33577 = t4248 * t8461;
    let t33578 = F::new(2.0) * t33577;
    let t33579 = t7732 * t8461;
    let t33580 = F::new(2.0) * t33579;
    let t33581 = t1843 * t8460;
    let t33582 = t651 * t33581;
    let t33583 = F::new(2.0) * t33582;
    let t33602 = t1931 * t1518;
    (t33183, t33578, t33580, t33581, t33583, t33602)
}
