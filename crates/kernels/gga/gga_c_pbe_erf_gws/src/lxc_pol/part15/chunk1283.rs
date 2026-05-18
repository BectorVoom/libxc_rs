//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1283/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1283<F: Float>(t14404: F, t19704: F, t51756: F, t51758: F, t51769: F, t51771: F, t51781: F, t51788: F, t53843: F, t53846: F, t53848: F, t53852: F, t53856: F, t53862: F, t53867: F, t53870: F) -> F {
    let t53872 = F::new(7.0) / F::new(144.0) * t51756 - F::new(7.0) / F::new(72.0) * t51758 + F::new(7.0) / F::new(48.0) * t51769 - t53843 / F::new(8.0) - F::new(7.0) / F::new(2304.0) * t51771 + t53846 / F::new(24.0) + t53848 / F::new(48.0) + F::new(7.0) / F::new(144.0) * t51781 + F::new(7.0) / F::new(288.0) * t51788 - F::new(35.0) / F::new(432.0) * t53852 + t53856 / F::new(384.0) + t19704 * t14404 / F::new(48.0) + t53862 / F::new(192.0) + F::new(5.0) / F::new(192.0) * t53867 - t53870 / F::new(1536.0);
    t53872
}
