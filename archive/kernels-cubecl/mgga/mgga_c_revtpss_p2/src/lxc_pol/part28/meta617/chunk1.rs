//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2159/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2159<F: Float>(t98949: F, t14788: F, t25270: F, t14682: F, t14804: F, t27261: F, t14793: F, t92952: F, t92956: F, t98937: F, t98940: F, t98943: F, t98945: F, t98947: F) -> F {
    let t98950 = F::cast_from(0.16006300097412701803e-1_f64) * t98949;
    let t98951 = t25270 * t14788;
    let t98953 = t25270 * t14682;
    let t98955 = t27261 * t14804;
    let t98957 = t25270 * t14793;
    let t98959 = -F::cast_from(0.80031500487063509015e-2_f64) * t98937 - F::cast_from(0.16006300097412701803e-1_f64) * t92952 - F::cast_from(0.42874018118069736972e-3_f64) * t98940 + F::cast_from(0.2032800112371413129e-3_f64) * t92956 - F::cast_from(0.85748036236139473945e-2_f64) * t98943 + F::cast_from(0.17149607247227894789e-2_f64) * t98945 - F::cast_from(0.34299214494455789578e-2_f64) * t98947 - t98950 - F::cast_from(0.17149607247227894789e-1_f64) * t98951 - F::cast_from(0.42874018118069736972e-3_f64) * t98953 - F::cast_from(0.68598428988911579156e-2_f64) * t98955 + F::cast_from(0.34299214494455789578e-2_f64) * t98957;
    t98959
}
