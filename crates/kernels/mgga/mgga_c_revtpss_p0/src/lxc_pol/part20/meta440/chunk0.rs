//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1671/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1671<F: Float>(t3555: F, t3727: F, t13180: F, t493: F, t225: F, t3738: F, t3790: F, t1209: F, t13107: F, t460: F, t1269: F, t13043: F) -> (F, F, F, F, F, F, F) {
    let t45545 = t3555 * t3727;
    let t45551 = F::new(1.0) / t13180 / t493;
    let t45552 = t225 * t45551;
    let t45553 = t3738 * t3738;
    let t45559 = t3790 * t3790;
    let t45568 = t1209 * t13107;
    let t45575 = t460 * t13107;
    let t45584 = t1269 * t13043;
    (t45545, t45552, t45553, t45559, t45568, t45575, t45584)
}
