//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1294/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1294<F: Float>(t20019: F, t33294: F, t7292: F, t11061: F, t14555: F, t32214: F, t739: F, t1890: F, t3487: F, t5241: F, t23000: F, t7805: F) -> (F, F, F, F, F) {
    let t33297 = F::new(0.95334639871601137784e0) * t33294 * t20019 * t7292;
    let t33299 = F::new(0.15337170381568299871e1) * t14555 * t11061;
    let t33300 = t739 * t32214;
    let t33304 = t1890 * t32214;
    let t33308 = t5241 * t3487;
    let t33310 = t23000 * t33308 * t7805;
    (t33297, t33299, t33300, t33304, t33310)
}
