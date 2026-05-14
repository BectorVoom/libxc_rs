//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 603/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk603<F: Float>(t2059: F, t554: F, t8078: F, t8074: F, t8082: F, t8086: F, t8091: F, t8094: F, t8096: F, t8099: F, t8104: F, t8107: F, t8110: F, t8113: F, t8116: F, t8123: F, t8127: F, t8131: F, t8133: F, t8135: F, t8137: F) -> (F, F) {
    let t8909 = t2059 * t554;
    let t8914 = 0.18521666970164609055e-1 * t8078;
    let t8932 = 0.88904001456790123462e-1 * t8074 + t8914 - 0.22818693707242798355e1 * t8082 + 0.48897200801234567904e0 * t8086 + 0.10001700163888888889e0 * t8091 - 0.10001700163888888889e0 * t8094 + 0.26671200437037037038e0 * t8096 - 0.33339000546296296299e-1 * t8099 - 0.13335600218518518519e0 * t8104 + 0.66678001092592592595e-1 * t8107 - 0.11113000182098765433e-1 * t8110 + 0.16669500273148148149e-1 * t8113 + 0.22226000364197530866e-1 * t8116 + 0.51860667516460905352e-1 * t8123 + 0.16669500273148148149e-1 * t8127 + 0.48897200801234567904e0 * t8131 - 0.88904001456790123462e-1 * t8133 - 0.13335600218518518519e0 * t8135 - 0.17780800291358024692e0 * t8137;
    (t8909, t8932)
}
