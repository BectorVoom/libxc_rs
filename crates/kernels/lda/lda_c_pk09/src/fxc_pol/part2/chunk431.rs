//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 431/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk431<F: Float>(t2474: F, t359: F, t355: F, t347: F, t48: F, t292: F, t1265: F) -> (F, F, F, F, F, F) {
    let t2491 = t359 * t2474;
    let t2494 = t355 * t2474;
    let t2497 = t347 * t2474;
    let t2500 = t48 * t2474;
    let t2501 = t292 * t2500;
    let t2502 = t1265 * t2501;
    (t2491, t2494, t2497, t2500, t2501, t2502)
}
