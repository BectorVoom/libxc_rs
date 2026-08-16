//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 695/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk695(t6586: f64, t68: f64, t6302: f64, t1837: f64, t747: f64, t1992: f64, t1843: f64, t1995: f64, t1675: f64, t1972: f64, t567: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6587 = t6586 * t68;
    let t6588 = t6587 * t6302;
    let t6589 = 44.15969676259812_f64 * t6588;
    let t6590 = t747 * t1837;
    let t6591 = t1992 * t6590;
    let t6592 = 7.200326855928252_f64 * t6591;
    let t6593 = t747 * t1843;
    let t6594 = t1995 * t6593;
    let t6598 = t1972 * t1675;
    let t6600 = t567 * t95;
    (t6588, t6589, t6590, t6591, t6592, t6593, t6594, t6598, t6600)
}
