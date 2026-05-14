//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 908/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk908<F: Float>(t3134: F, t6188: F, t343: F, t8840: F, t337: F, t2121: F, t2134: F, t6445: F, t6447: F, t6481: F, t9007: F, t9011: F, t9015: F, t9018: F, t9019: F, t9021: F, t9023: F) -> (F, F, F, F, F, F, F) {
    let t9025 = t6188 * t3134 / 96.0;
    let t9026 = t8840 * t343;
    let t9027 = t337 * t9026;
    let t9028 = t2121 * t9027;
    let t9030 = t2134 * t9028 / 48.0;
    let t9031 = 7.0 / 288.0 * t6445;
    let t9032 = 7.0 / 288.0 * t6447;
    let t9033 = 35.0 / 108.0 * t6481;
    let t9034 = t9007 - t9011 - t9015 + t9018 - t9019 - t9021 - t9023 - t9025 - t9030 + t9031 + t9032 - t9033;
    (t9025, t9026, t9030, t9031, t9032, t9033, t9034)
}
