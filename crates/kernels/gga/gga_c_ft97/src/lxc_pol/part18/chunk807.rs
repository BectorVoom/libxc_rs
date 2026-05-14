//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 807/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk807<F: Float>(t1712: F, t5546: F, t22652: F, t428: F, t5517: F, t5533: F, t1681: F, t52: F, t67: F, t11: F, t1675: F, t41: F, t78: F, t388: F, t1685: F, t38: F) -> (F, F, F, F, F, F, F) {
    let t22657 = t5546 * t1712;
    let t22661 = t22652 * t428;
    let t22667 = t5517 * t5533;
    let t22673 = t52 * t67 * t1681;
    let t22675 = 0.44057546758024691357e0 * t41 * t11 * t1675 + 0.37540077436335915589e-1 * t22673;
    let t22676 = t78 * t22675;
    let t22677 = t388 * t22676;
    let t22679 = t38 * t1685;
    (t22657, t22661, t22667, t22673, t22675, t22677, t22679)
}
