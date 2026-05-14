//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 942/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk942<F: Float>(t26916: F, t33468: F, t26904: F, t33517: F, t7642: F, t33494: F, t3567: F, t1209: F, t124664: F, t26948: F, t7657: F, t33397: F, t33424: F, t12916: F, t33414: F, t33416: F) -> (F, F, F, F, F, F, F) {
    let t124671 = t33468 * t26916;
    let t124675 = t7642 * t26904 * t33517;
    let t124684 = t3567 * t33494;
    let t124694 = t1209 * t124664;
    let t124706 = t26948 * t7657;
    let t124711 = t33397 * t33424;
    let t124717 = t33414 * t12916 * t33416;
    (t124671, t124675, t124684, t124694, t124706, t124711, t124717)
}
