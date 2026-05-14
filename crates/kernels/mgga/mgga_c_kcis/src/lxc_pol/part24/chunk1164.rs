//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1164/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1164<F: Float>(t100001: F, t100003: F, t100005: F, t100007: F, t100009: F, t100011: F, t100013: F, t100015: F, t100017: F, t100019: F, t100021: F, t99984: F, t99986: F, t99988: F, t99990: F, t99992: F, t99994: F, t99997: F, t99999: F) -> (F,) {
    let t101701 = 0.9375e-1 * t99984 - 0.1875e0 * t99986 + 0.20234375e-1 * t99988 + 0.5e0 * t99990 + 0.28777777777777777778e0 * t99992 - 0.809375e-1 * t99994 - 0.9375e-1 * t99997 - 0.41666666666666666667e-1 * t99999 - 0.25e0 * t100001 - 0.26979166666666666667e-1 * t100003 + 0.41666666666666666667e-1 * t100005 + 0.21583333333333333333e0 * t100007 + 0.53958333333333333333e-1 * t100009 - 0.125e0 * t100011 + 0.20833333333333333333e-1 * t100013 - 0.5625e0 * t100015 + 0.375e0 * t100017 + 0.53958333333333333334e-1 * t100019 - 0.26979166666666666667e-1 * t100021;
    (t101701,)
}
