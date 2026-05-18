//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1115/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1115<F: Float>(t14072: F, t14084: F, t14055: F, t14059: F, t14061: F, t14065: F, t14067: F, t14070: F, t14074: F, t14076: F, t14080: F, t14086: F, t14088: F, t14094: F, t14097: F, t14103: F) -> (F, F, F) {
    let t14229 = F::new(119.0) / F::new(3456.0) * t14072;
    let t14233 = F::new(35.0) / F::new(216.0) * t14084;
    let t14239 = F::new(5.0) / F::new(192.0) * t14055 + F::new(7.0) / F::new(144.0) * t14059 - t14061 / F::new(192.0) - t14065 / F::new(12.0) + t14067 / F::new(192.0) - t14070 / F::new(24.0) + t14229 - t14074 / F::new(384.0) - t14076 / F::new(384.0) + F::new(7.0) / F::new(288.0) * t14080 + t14233 + t14086 / F::new(384.0) + t14088 / F::new(384.0) - t14094 / F::new(48.0) - t14097 / F::new(48.0) + t14103 / F::new(24.0);
    (t14229, t14233, t14239)
}
