//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 411/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk411(t1307: f64, t1413: f64, t1465: f64, t1532: f64, t1125: f64, t153: f64, t274: f64, t474: f64, t678: f64, t1089: f64, t1197: f64, t1199: f64, t1202: f64, t1203: f64, t1206: f64, t1209: f64, t1213: f64, t1215: f64, t156: f64, t168: f64, t242: f64, t245: f64) -> (f64, f64, f64, f64) {
    let t1534 = t1307 + t1413 + t1465 + t1532;
    let t1540 = 1.328721022894618_f64 * t153 * t1125 * t274;
    let t1542 = t153 * t474 * t678;
    let t1547 = -t1197 + 0.1675256410710088_f64 * t1199 + t1202 - 0.0837628205355044_f64 * t1203 * t242 - 0.1675256410710088_f64 * t1206 - t1209 - t1213 + 0.039794582218349216_f64 * t1215 - 0.011938374665504766_f64 * t168 * t245 * t1534 + t1540 - 1.1389037339096726_f64 * t1542 + 0.42708890021612717_f64 * t153 * t156 * t1089;
    (t1534, t1540, t1542, t1547)
}
