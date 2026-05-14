//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 964/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk964<F: Float>(t1079: F, t1080: F, t16612: F, t184: F, t20044: F, t20995: F, t20996: F, t21: F, t21002: F, t21005: F, t21008: F, t21087: F, t21088: F, t3601: F, t4888: F, t4890: F, t4895: F, t623: F, t78929: F, t87868: F, t87906: F, t87941: F, t88021: F, t920: F) -> (F,) {
    let t88051 = t623 * (t87868 + t87906 + t87941 + t88021) * t184 * t21 / 4.0 + t623 * t1079 * t20044 + 3.0 * t3601 * t21008 + t78929 * t1080 + t3601 * t20996 + 3.0 * t3601 * t21005 + t623 * t21087 * t920 + 3.0 * t623 * t1079 * t920 * t4888 + 3.0 / 2.0 * t16612 * t4895 + 3.0 / 2.0 * t16612 * t4890 + t623 * t20995 * t920 + 3.0 * t3601 * t21002 + t3601 * t21088;
    (t88051,)
}
