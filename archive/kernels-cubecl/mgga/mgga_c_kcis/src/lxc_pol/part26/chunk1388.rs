//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1388/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1388<F: Float>(t102985: F, t102987: F, t102989: F, t102991: F, t102993: F, t102995: F, t102997: F, t102999: F, t103002: F, t103004: F, t103006: F, t103031: F, t103033: F, t103035: F, t103038: F, t103040: F, t103043: F, t103046: F, t103049: F, t103051: F, t103053: F, t103056: F) -> (F, F) {
    let t103870 = t102985 / F::cast_from(288.0_f64) - t102987 / F::cast_from(64.0_f64) - t102989 / F::cast_from(9.0_f64) + t102991 / F::cast_from(432.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t102993 - t102995 / F::cast_from(36.0_f64) + F::cast_from(19.0_f64) / F::cast_from(72.0_f64) * t102997 - t102999 / F::cast_from(72.0_f64) - t103002 / F::cast_from(32.0_f64) + t103004 / F::cast_from(4.0_f64) + t103006 / F::cast_from(3.0_f64);
    let t103894 = t103031 / F::cast_from(16.0_f64) - t103033 / F::cast_from(8.0_f64) - t103035 / F::cast_from(96.0_f64) - t103038 / F::cast_from(16.0_f64) + t103040 / F::cast_from(3.0_f64) - t103043 / F::cast_from(16.0_f64) + t103046 / F::cast_from(3.0_f64) - F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t103049 + t103051 / F::cast_from(48.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t103053 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t103056;
    (t103870, t103894)
}
