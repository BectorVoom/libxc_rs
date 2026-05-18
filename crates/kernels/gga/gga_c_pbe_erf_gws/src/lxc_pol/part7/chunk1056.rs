//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1056/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1056<F: Float>(t19059: F, t19062: F, t19064: F, t19066: F, t19068: F, t19072: F, t19075: F, t19077: F, t19079: F, t19081: F, t522: F, t5621: F) -> (F, F) {
    let t19083 = -F::new(28.0) / F::new(81.0) * t19059 + F::new(8.0) / F::new(9.0) * t19062 - t19064 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t19066 + t19068 / F::new(3.0) - F::new(28.0) / F::new(81.0) * t19072 + F::new(8.0) / F::new(9.0) * t19075 - t19077 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t19079 + t19081 / F::new(3.0);
    let t19087 = t522 * t5621;
    (t19083, t19087)
}
