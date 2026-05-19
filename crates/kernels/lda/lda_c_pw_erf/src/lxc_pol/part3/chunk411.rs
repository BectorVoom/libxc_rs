//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 411/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk411<F: Float>(t1307: F, t1413: F, t1465: F, t1532: F, t1125: F, t153: F, t274: F, t474: F, t678: F, t1089: F, t1197: F, t1199: F, t1202: F, t1203: F, t1206: F, t1209: F, t1213: F, t1215: F, t156: F, t168: F, t242: F, t245: F) -> (F, F, F, F) {
    let t1534 = t1307 + t1413 + t1465 + t1532;
    let t1540 = F::cast_from(1.328721022894618_f64) * t153 * t1125 * t274;
    let t1542 = t153 * t474 * t678;
    let t1547 = -t1197 + F::cast_from(0.1675256410710088_f64) * t1199 + t1202 - F::cast_from(0.0837628205355044_f64) * t1203 * t242 - F::cast_from(0.1675256410710088_f64) * t1206 - t1209 - t1213 + F::cast_from(0.039794582218349216_f64) * t1215 - F::cast_from(0.011938374665504766_f64) * t168 * t245 * t1534 + t1540 - F::cast_from(1.1389037339096726_f64) * t1542 + F::cast_from(0.42708890021612717_f64) * t153 * t156 * t1089;
    (t1534, t1540, t1542, t1547)
}
