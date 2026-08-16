//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 905/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk905(t355: f64, t9602: f64, t1279: f64, t2487: f64, t359: f64, t1303: f64, t1310: f64, t1287: f64, t1283: f64, t2501: f64, t5042: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9603 = t355 * t9602;
    let t9606 = t1279 * t2487;
    let t9609 = t359 * t9602;
    let t9612 = t1303 * t2487;
    let t9615 = t1310 * t9602;
    let t9616 = t9615 * t1287;
    let t9618 = t1283 * t9602;
    let t9619 = t9618 * t1287;
    let t9623 = t5042 * t2501;
    (t9603, t9606, t9609, t9612, t9616, t9619, t9623)
}
