//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 835/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk835<F: Float>(t41738: F, t6716: F, t6717: F, t12875: F, t18651: F, t40103: F, t10526: F, t20471: F, t10241: F, t2293: F) -> (F, F, F, F, F) {
    let t41741 = F::new(0.62115540045351614476e2) * t6716 * t6717 * t41738;
    let t41743 = F::new(0.27606906686822939767e2) * t18651 * t12875;
    let t41744 = F::new(0.23005755572352449806e1) * t40103;
    let t41747 = F::new(0.21450293971110256001e1) * t20471 * t10526 * t41738;
    let t41749 = t10241 * t2293;
    (t41741, t41743, t41744, t41747, t41749)
}
