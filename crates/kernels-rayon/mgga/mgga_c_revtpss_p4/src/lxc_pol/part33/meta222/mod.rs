//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1013;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta222(t508: f64, t5920: f64, t4303: f64, t4306: f64, t2498: f64, t2518: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2628: f64, t2632: f64, t45: f64, t57: f64, t4397: f64, t2375: f64, t5819: f64, t5825: f64, t78: f64, t2382: f64, t81: f64, t162: f64, t187: f64, t150: f64, t190: f64, t1522: f64, t4311: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5921, t5924, t5925, t5926) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1013(t508, t5920, t4303, t4306, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2628, t2632);
        let (t5927, t5940, t5941, t5943, t5944, t5945, t5947) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1014(t45, t57, t4397, t2375, t5819, t5825, t78, t2382, t81, t162, t187, t150, t190, t1522, t4311, zeta_threshold);
    (t5921, t5924, t5925, t5926, t5927, t5940, t5941, t5943, t5944, t5945, t5947)
}
