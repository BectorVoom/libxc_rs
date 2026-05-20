//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta672<F: Float>(t3046: F, t4980: F, t12046: F, t989: F, t1035: F, t42859: F, t342: F, t12166: F, t16409: F, t994: F, t3057: F, t11223: F, t3286: F) -> (F, F, F, F, F, F, F, F) {
        let (t43360, t43384, t43400, t43401, t43420, t43432, t43438, t43443) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2408::<F>(t3046, t4980, t12046, t989, t1035, t42859, t342, t12166, t16409, t994, t3057, t11223, t3286);
    (t43360, t43384, t43400, t43401, t43420, t43432, t43438, t43443)
}
