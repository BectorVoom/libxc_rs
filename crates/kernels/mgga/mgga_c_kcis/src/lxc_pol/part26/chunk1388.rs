//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1388/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1388<F: Float>(t102985: F, t102987: F, t102989: F, t102991: F, t102993: F, t102995: F, t102997: F, t102999: F, t103002: F, t103004: F, t103006: F, t103031: F, t103033: F, t103035: F, t103038: F, t103040: F, t103043: F, t103046: F, t103049: F, t103051: F, t103053: F, t103056: F) -> (F, F) {
    let t103870 = t102985 / F::new(288.0) - t102987 / F::new(64.0) - t102989 / F::new(9.0) + t102991 / F::new(432.0) + F::new(2.0) / F::new(9.0) * t102993 - t102995 / F::new(36.0) + F::new(19.0) / F::new(72.0) * t102997 - t102999 / F::new(72.0) - t103002 / F::new(32.0) + t103004 / F::new(4.0) + t103006 / F::new(3.0);
    let t103894 = t103031 / F::new(16.0) - t103033 / F::new(8.0) - t103035 / F::new(96.0) - t103038 / F::new(16.0) + t103040 / F::new(3.0) - t103043 / F::new(16.0) + t103046 / F::new(3.0) - F::new(11.0) / F::new(18.0) * t103049 + t103051 / F::new(48.0) + F::new(2.0) / F::new(9.0) * t103053 - F::new(2.0) / F::new(9.0) * t103056;
    (t103870, t103894)
}
