//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1078/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1078(t11749: f64, t1818: f64, t2052: f64, t2795: f64, t7080: f64, t11096: f64, t11711: f64, t11715: f64, t11717: f64, t11721: f64, t11723: f64, t11733: f64, t1748: f64, t2032: f64, t2071: f64, t2085: f64, t2091: f64, t2783: f64, t2796: f64, t453: f64, t6288: f64) -> (f64, f64) {
    let t11750 = t11749 * t1818;
    let t11751 = t11750 * t2052;
    let t11754 = t2795 * t7080;
    let t11759 = t11711 * t11096 / 3.0_f64 + t11715 * t11717 / 6.0_f64 + t11721 / 6.0_f64 + t11723 * t2032 / 6.0_f64 - t2796 * t6288 / 6.0_f64 + t2085 * t2783 / 6.0_f64 + t2091 * t2783 / 6.0_f64 + t453 * t11733 / 6.0_f64 - t11751 * t1748 / 6.0_f64 - t11754 * t1748 / 6.0_f64 - t2071 * t2783 / 6.0_f64;
    (t11750, t11759)
}
