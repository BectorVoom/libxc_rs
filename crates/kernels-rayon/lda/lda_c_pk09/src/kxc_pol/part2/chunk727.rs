//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 727/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk727(t545: f64, t6287: f64, t1744: f64, t337: f64, t429: f64, t1741: f64, t2042: f64, t1468: f64, t544: f64, t1747: f64, t6302: f64, t1838: f64, t7273: f64) -> (f64, f64, f64, f64, f64) {
    let t7469 = t545 * t6287;
    let t7472 = t1744 * t337;
    let t7473 = t7472 * t429;
    let t7474 = t1741 * t7473;
    let t7475 = t7474 * t2042;
    let t7481 = t544 * t1468;
    let t7482 = t7481 * t1747;
    let t7484 = 4.855032390388656_f64 * t7482 * t6302;
    let t7485 = t7273 * t1838;
    (t7469, t7473, t7475, t7484, t7485)
}
