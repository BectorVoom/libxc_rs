//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1222/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1222<F: Float>(t97702: F, t97704: F, t97707: F, t97709: F, t97711: F, t97713: F, t97715: F, t97717: F, t97719: F, t97721: F, t97723: F, t97725: F, t97728: F, t97730: F, t97732: F, t97734: F, t97736: F, t97738: F) -> F {
    let t97919 = -t97702 / F::new(8.0) + t97704 / F::new(64.0) + F::new(3.0) / F::new(64.0) * t97707 - t97709 / F::new(72.0) + t97711 / F::new(24.0) - t97713 / F::new(24.0) - t97715 / F::new(12.0) - F::new(3.0) / F::new(8.0) * t97717 - t97719 / F::new(12.0) + t97721 / F::new(12.0) + t97723 / F::new(27.0) - t97725 / F::new(16.0) + t97728 / F::new(3.0) + t97730 / F::new(12.0) + t97732 / F::new(24.0) - t97734 / F::new(8.0) + t97736 / F::new(288.0) - t97738 / F::new(64.0);
    t97919
}
