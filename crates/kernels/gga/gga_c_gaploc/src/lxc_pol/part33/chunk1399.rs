//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1399/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1399<F: Float>(t1445: F, t1457: F, t31124: F, t31126: F, t31129: F, t31131: F, t31135: F, t31144: F, t31160: F, t31163: F, t31166: F, t31169: F, t31172: F, t31175: F, t34922: F, t38388: F, t38429: F, t4540: F, t567: F) -> F {
    let t38787 = -t34922 + t31124 - F::new(0.15337170381568299871e1) * t31126 + F::new(0.72851559312449424385e1) * t31129 + F::new(0.10224780254378866581e1) * t31131 + t31135 + F::new(0.30674340763136599742e1) * t31144 + t31160 - t31163 - t31166 - t31169 - F::new(0.21450293971110256001e1) * t4540 * t1457 * t38429 + F::new(0.23005755572352449806e1) * t567 * t1445 * t38388 - t31172 - t31175;
    t38787
}
