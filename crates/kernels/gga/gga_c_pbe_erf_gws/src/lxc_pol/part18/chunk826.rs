//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 826/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk826<F: Float>(t10063: F, t128: F, t10: F, t156: F, t3656: F, t496: F, t3660: F, t3665: F, t501: F, t395: F, t3668: F, t481: F, t2873: F, t978: F, t10051: F, t10054: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5764: F, t5776: F, t8126: F, t8137: F, t8139: F, t8142: F) -> (F, F, F, F) {
    let t10064 = t128 * t10063;
    let t10065 = t10 * t10064;
    let t10068 = t156 * t3656;
    let t10069 = t496 * t10068;
    let t10071 = t156 * t3660;
    let t10072 = t496 * t10071;
    let t10074 = t501 * t3665;
    let t10075 = t10074 * t395;
    let t10077 = t501 * t3668;
    let t10078 = t10077 * t395;
    let t10081 = t3665 * t481;
    let t10085 = t978 * t2873;
    let t10089 = -t5749 - t5751 + t5753 - t5755 - t5759 - t10051 + t10054 - t496 * t10065 / 2.0 - t10069 / 2.0 + t10072 / 6.0 - 0.293808e1 * t10075 + 0.73452e0 * t10078 - 0.48968000000000000001e0 * t5764 - 6.0 * t496 * t10 * t10081 + 3.0 * t496 * t10 * t10085 - t5776 - t8126 - t8137 - t8139 + t8142;
    (t10065, t10068, t10071, t10089)
}
