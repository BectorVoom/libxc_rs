//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 459/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk459(t2474: f64, t359: f64, t355: f64, t347: f64, t48: f64, t292: f64, t1265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2491 = t359 * t2474;
    let t2494 = t355 * t2474;
    let t2497 = t347 * t2474;
    let t2500 = t48 * t2474;
    let t2501 = t292 * t2500;
    let t2502 = t1265 * t2501;
    (t2491, t2494, t2497, t2500, t2501, t2502)
}
