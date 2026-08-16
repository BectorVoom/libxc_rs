//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 856/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk856(t609: f64, t8732: f64, t891: f64, t4587: f64, t623: f64, t896: f64, t633: f64, t903: f64, t143: f64, t151: f64, t2206: f64, t3744: f64, t4475: f64, t8096: f64, t8101: f64, t8760: f64, t8816: f64, t8821: f64, t8829: f64, t933: f64, t98: f64) -> f64 {
    let t8836 = t891 * t8732 * t609;
    let t8837 = t4587 * t8836;
    let t8840 = t896 * t8732 * t623;
    let t8844 = t903 * t8732 * t633;
    let t8848 = 3.7610742193750633_f64 * t8760 + 1.8805371096875316_f64 * t8816 * t98 + 22.07984838129906_f64 * t8821 + 3.7610742193750633_f64 * t143 * t8096 + 3.7610742193750633_f64 * t143 * t8101 - 0.6268457032291772_f64 * t933 * t2206 - 1.8805371096875316_f64 * t8829 - 1.8805371096875316_f64 * t151 * t8096 - 1.8805371096875316_f64 * t151 * t8101 + 1.1846959580306418_f64 * t8837 + 1.1846959580306418_f64 * t3744 * t8840 + 1.1846959580306418_f64 * t3744 * t8844 + 22.07984838129906_f64 * t4475;
    t8848
}
