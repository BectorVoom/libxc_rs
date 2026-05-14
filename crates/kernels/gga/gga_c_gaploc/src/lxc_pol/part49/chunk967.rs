//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 967/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk967<F: Float>(t13892: F, t5676: F, t12161: F, t2033: F, t2365: F, t2610: F, t13848: F, t7416: F, t12255: F, t769: F, t3470: F, t313: F, t39403: F, t44085: F, t44089: F, t44092: F, t44093: F, t44097: F, t44099: F, t47486: F) -> (F,) {
    let t47488 = t5676 * t13892;
    let t47492 = t2033 * t2365 * t2610 * t12161;
    let t47494 = t7416 * t13848;
    let t47496 = t769 * t12255;
    let t47497 = t47496 * t3470;
    let t47500 = t313 * t39403;
    let t47501 = t47500 * t3470;
    let t47503 = -t44085 - t44089 - 0.79445533226334281487e-1 * t47486 - 0.14896037479937677779e-1 * t47488 - 0.14896037479937677779e-1 * t47492 + 0.19171462976960374838e0 * t47494 - 0.10725146985555128001e1 * t47497 - t44092 - 0.69017266717057349418e1 * t44093 - t44097 - t44099 - 0.10725146985555128001e1 * t47501;
    (t47503,)
}
