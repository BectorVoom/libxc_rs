//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 694/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk694<F: Float>(t302: F, t668: F, t683: F, t14953: F, t14955: F, t14957: F, t14958: F, t14962: F, t14965: F, t14968: F, t14971: F, t14974: F, t14977: F, t14980: F, t14983: F, t14986: F, t14989: F, t14992: F, t14995: F, t14999: F, t15000: F, t15004: F, t3051: F, t3139: F, t462: F, t92: F) -> (F,) {
    let t15007 = t683 * t302 * t668;
    let t15010 = -t14953 - t14955 + t14957 - 2.0 / 9.0 * t462 * t14958 - 10.0 / 27.0 * t462 * t14962 + 8.0 / 9.0 * t3139 * t14965 + t462 * t14968 / 3.0 + 4.0 / 3.0 * t462 * t14971 - 2.0 / 3.0 * t462 * t14974 - 2.0 * t462 * t14977 - 2.0 / 3.0 * t462 * t14980 - 4.0 / 3.0 * t3139 * t14983 + 2.0 / 3.0 * t462 * t14986 - 8.0 / 3.0 * t3139 * t14989 + t462 * t14992 / 3.0 + 2.0 / 9.0 * t462 * t14995 - t14999 + 2.0 / 3.0 * t462 * t15000 - t92 * t15004 + 2.0 / 3.0 * t3051 * t15007;
    (t15010,)
}
