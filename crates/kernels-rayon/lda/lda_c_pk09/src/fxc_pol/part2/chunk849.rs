//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 849/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk849(t3743: f64, t4673: f64, t2192: f64, t62: f64, t694: f64, t891: f64, t133: f64, t7766: f64, t742: f64, t7704: f64, t947: f64, t131: f64) -> (f64, f64, f64, f64, f64) {
    let t8731 = t4673 * t3743;
    let t8732 = t62 * t2192;
    let t8734 = t891 * t8732 * t694;
    let t8743 = t133 * t7766;
    let t8744 = t742 * t8743;
    let t8747 = t947 * t7704;
    let t8748 = t131 * t8747;
    (t8731, t8732, t8734, t8744, t8748)
}
