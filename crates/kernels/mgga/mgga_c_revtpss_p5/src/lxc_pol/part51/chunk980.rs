//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 980/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk980<F: Float>(t33577: F, t7732: F, t8461: F, t1843: F, t8460: F, t651: F, t1518: F, t8557: F, t4248: F, t8457: F, t1936: F, t7883: F) -> (F, F, F, F, F, F, F, F) {
    let t33578 = F::new(2.0) * t33577;
    let t33579 = t7732 * t8461;
    let t33580 = F::new(2.0) * t33579;
    let t33581 = t1843 * t8460;
    let t33582 = t651 * t33581;
    let t33583 = F::new(2.0) * t33582;
    let t33584 = t8557 * t1518;
    let t33587 = t4248 * t8457;
    let t33589 = t7732 * t8457;
    let t33591 = t7883 * t1936;
    (t33578, t33580, t33581, t33583, t33584, t33587, t33589, t33591)
}
