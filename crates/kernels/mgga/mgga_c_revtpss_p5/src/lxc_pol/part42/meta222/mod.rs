//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk862;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta222<F: Float>(t508: F, t5920: F, t4303: F, t4306: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2628: F, t2632: F, t45: F, t57: F, t4397: F, t2375: F, t5819: F, t5825: F, t78: F, t2382: F, t81: F, t162: F, t187: F, t150: F, t190: F, t1522: F, t4311: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5921, t5924, t5925, t5926) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk862::<F>(t508, t5920, t4303, t4306, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2628, t2632);
        let (t5927, t5940, t5941, t5943, t5944, t5945, t5947) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk863::<F>(t45, t57, t4397, t2375, t5819, t5825, t78, t2382, t81, t162, t187, t150, t190, t1522, t4311, zeta_threshold);
    (t5921, t5924, t5925, t5926, t5927, t5940, t5941, t5943, t5944, t5945, t5947)
}
