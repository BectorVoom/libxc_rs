//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 954/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk954(t1561: f64, t2487: f64, t2559: f64, t6022: f64, t1435: f64, t2555: f64, t2552: f64, t131: f64, t2143: f64, t309: f64, t319: f64, t2606: f64, t5747: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10044 = t1561 * t2487;
    let t10048 = t2559 * t6022;
    let t10050 = t2555 * t1435;
    let t10052 = t2552 * t1435;
    let t10059 = t309 * t131 * t2143;
    let t10060 = t319 * t10059;
    let t10062 = t2606 * t5747;
    (t10044, t10048, t10050, t10052, t10059, t10060, t10062)
}
