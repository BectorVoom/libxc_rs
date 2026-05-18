//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1435/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1435<F: Float>(t224: F, t32721: F, t32741: F, t33983: F, t35243: F, t11142: F, t617: F, t10289: F, t10299: F, t10293: F, t10302: F, t10625: F) -> (F, F, F, F, F, F, F) {
    let t35246 = t224 * (t32721 + t32741 + t33983 + t35243);
    let t35247 = t617 * t11142;
    let t35252 = F::new(2.0) * t10289;
    let t35253 = F::new(4.0) * t10299;
    let t35254 = F::new(4.0) * t10293;
    let t35255 = F::new(4.0) * t10302;
    let t35256 = F::new(2.0) * t10625;
    (t35246, t35247, t35252, t35253, t35254, t35255, t35256)
}
