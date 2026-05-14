//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 640/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk640<F: Float>(t1139: F, t199: F, t566: F, t718: F, t2813: F, t1329: F, t1200: F, t391: F, t107: F, t110: F, t122: F, t1338: F, t202: F, t2804: F, t3974: F, t399: F, t4060: F, t4063: F, t4169: F, t4174: F, t4177: F, t4181: F, t4185: F, t4187: F, t4209: F, t84: F) -> (F, F, F, F, F, F) {
    let t4212 = t1139 * t199;
    let t4214 = t718 * t566;
    let t4216 = t2813 * t199;
    let t4218 = t1329 * t566;
    let t4220 = t391 * t1200;
    let t4228 = 0.42708890021612717 * t107 * t110 * t3974 - 1.7083556008645087 * t4060 + 3.9861630686838536 * t4063 - 0.011938374665504766 * t122 * t202 * t4169 - 0.15917832887339686 * t4174 + 0.05969187332752383 * t4177 - t4181 + t4185 + t4187 - 0.0837628205355044 * t84 * t4209 - 0.5025769232130264 * t4212 - 0.5025769232130264 * t4214 + 0.2512884616065132 * t4216 + 0.5025769232130264 * t4218 + 0.2512884616065132 * t4220 - 0.0837628205355044 * t2804 * t199 - 0.2512884616065132 * t1338 * t566 - 0.2512884616065132 * t399 * t1200;
    (t4212, t4214, t4216, t4218, t4220, t4228)
}
