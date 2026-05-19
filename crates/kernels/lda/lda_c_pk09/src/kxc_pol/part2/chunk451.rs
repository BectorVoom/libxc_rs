//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 451/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk451<F: Float>(t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t793: F, t794: F, t798: F, t799: F, t804: F, t89: F) -> (F, F, F) {
    let t2417 = t793 + t794 + F::cast_from(9.625452574583042_f64) * t2159 + F::cast_from(9.625452574583042_f64) * t2163 - F::cast_from(9.625452574583042_f64) * t2167 + t798 + t799 + F::new(0.64) * t2171 + F::new(0.64) * t2175 - F::new(0.64) * t2179;
    let t2418 = t2417 * t804;
    let t2419 = t2418 * t89;
    (t2417, t2418, t2419)
}
