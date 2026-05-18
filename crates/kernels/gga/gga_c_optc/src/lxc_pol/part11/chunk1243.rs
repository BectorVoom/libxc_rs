//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1243/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1243<F: Float>(t5: F, t56203: F, t127: F, t151: F, t2024: F, t2113: F, t2124: F, t2159: F, t3467: F, t48906: F, t48922: F, t48924: F, t48960: F, t48962: F, t48990: F, t48992: F, t56110: F, t56123: F, t56178: F, t56404: F, t673: F, t675: F, t696: F, t7129: F) -> F {
    let t56540 = t5 * t56203;
    let t56553 = -F::new(0.48681704342817043984e1) * t48906 - F::new(0.31295381363239528276e1) * t2124 * t7129 * t56123 + F::new(0.69545291918310062836e0) * t3467 * t151 * t56110 - F::new(0.33855833396020740576e1) * t48922 + F::new(0.9736340868563408797e1) * t48924 + F::new(0.3173984380876944429e0) * t2159 * t696 * t56178 - F::new(0.48681704342817043985e1) * t48960 - F::new(0.48681704342817043985e1) * t48962 - F::new(0.48681704342817043984e1) * t48990 - F::new(0.33855833396020740576e1) * t48992 + F::new(0.52158968938732547127e0) * t2113 * t675 * t56540 * t2024 - F::new(0.26079484469366273564e0) * t673 * t675 * t56540 * t127 - F::new(0.86931614897887578546e-1) * t673 * t675 * t56404 * t127;
    t56553
}
