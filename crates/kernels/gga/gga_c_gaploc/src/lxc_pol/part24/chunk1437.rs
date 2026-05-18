//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1437/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1437<F: Float>(t32099: F, t32102: F, t33952: F, t33961: F, t33966: F, t33968: F, t33974: F, t33979: F, t33981: F, t33986: F, t33997: F, t34006: F, t34008: F, t34012: F, t34018: F, t34023: F, t35239: F, t35240: F, t35246: F, t35247: F) -> F {
    let t39540 = t32099 - t32102 + t35246 + t33952 + t33961 + t33966 - t33968 - t33974 - t33979 - t33981 + t33986 + t33997 - t34006 + t34008 + t34012 + t35247 - t34018 + t34023 - t35239 - t35240;
    t39540
}
