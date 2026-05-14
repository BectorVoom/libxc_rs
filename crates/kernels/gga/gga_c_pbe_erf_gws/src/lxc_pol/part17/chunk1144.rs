//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1144/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1144<F: Float>(t51256: F, t54158: F, t54160: F, t54162: F, t54164: F, t54167: F, t54168: F, t54170: F, t54173: F, t54175: F, t54177: F, t54179: F, t14567: F, t2080: F, t9544: F, t9365: F) -> (F, F, F) {
    let t54181 = -t54158 / 48.0 - t54160 / 24.0 - t54162 / 192.0 + t54164 / 96.0 + t54167 + t54168 / 24.0 + t54170 / 48.0 + 7.0 / 144.0 * t51256 - t54173 / 96.0 + 5.0 / 192.0 * t54175 + t54177 / 96.0 - t54179 / 64.0;
    let t54183 = t2080 * t9544 * t14567;
    let t54186 = t2080 * t9365 * t14567;
    (t54181, t54183, t54186)
}
