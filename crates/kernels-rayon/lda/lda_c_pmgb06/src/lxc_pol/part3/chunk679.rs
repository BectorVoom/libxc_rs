//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 679/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk679(t1139: f64, t199: f64, t566: f64, t718: f64, t2813: f64, t1329: f64, t1200: f64, t391: f64, t107: f64, t110: f64, t122: f64, t1338: f64, t202: f64, t2804: f64, t3974: f64, t399: f64, t4060: f64, t4063: f64, t4169: f64, t4174: f64, t4177: f64, t4181: f64, t4185: f64, t4187: f64, t4209: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4212 = t1139 * t199;
    let t4214 = t718 * t566;
    let t4216 = t2813 * t199;
    let t4218 = t1329 * t566;
    let t4220 = t391 * t1200;
    let t4228 = 0.42708890021612717_f64 * t107 * t110 * t3974 - 1.7083556008645087_f64 * t4060 + 3.9861630686838536_f64 * t4063 - 0.011938374665504766_f64 * t122 * t202 * t4169 - 0.15917832887339686_f64 * t4174 + 0.05969187332752383_f64 * t4177 - t4181 + t4185 + t4187 - 0.0837628205355044_f64 * t84 * t4209 - 0.5025769232130264_f64 * t4212 - 0.5025769232130264_f64 * t4214 + 0.2512884616065132_f64 * t4216 + 0.5025769232130264_f64 * t4218 + 0.2512884616065132_f64 * t4220 - 0.0837628205355044_f64 * t2804 * t199 - 0.2512884616065132_f64 * t1338 * t566 - 0.2512884616065132_f64 * t399 * t1200;
    (t4212, t4214, t4216, t4218, t4220, t4228)
}
