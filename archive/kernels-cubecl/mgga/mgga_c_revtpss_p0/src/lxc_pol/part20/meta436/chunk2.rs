//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1645/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1645<F: Float>(t422: F, t44999: F, t45012: F, t44087: F, t44096: F, t44100: F, t44103: F, t44106: F, t44108: F, t44111: F, t44114: F, t44122: F, t44984: F, t44987: F) -> (F, F) {
    let t45015 = F::cast_from(0.621814e-1_f64) * (t44999 + t45012) * t422;
    let t45016 = t44087 + t44096 + t44100 - t44103 + t44106 + t44108 - t44111 - t44114 + t44122 + t44984 - t44987 - t45015;
    (t45015, t45016)
}
