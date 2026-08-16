//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1358/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1358(t1444: f64, t1943: f64, t5654: f64, t98359: f64, t102011: f64, t102014: f64, t102017: f64, t103083: f64, t103224: f64, t103240: f64, t103251: f64, t103255: f64, t103258: f64, t28388: f64, t28392: f64, t28439: f64, t3984: f64, t59401: f64, t7908: f64, t7909: f64, t94227: f64) -> (f64, f64) {
    let t103263 = t98359 * t1943 * t1444 * t5654;
    let t103268 = 0.185671721767578125e-4_f64 * t28388 * t103224 + 0.111403033060546875e-3_f64 * t28388 * t103083 + 0.20612155671296296296e-4_f64 * t103240 - 0.33163888888888888888e-2_f64 * t102011 - 0.33163888888888888888e-2_f64 * t102014 + 0.66327777777777777776e-2_f64 * t102017 + 0.23168402777777777778e-3_f64 * t7908 * t3984 * t7909 * t59401 + 0.23168402777777777778e-3_f64 * t7908 * t103251 + 0.15445601851851851852e-3_f64 * t103255 - 0.61836467013888888889e-4_f64 * t94227 * t103258 - 0.12367293402777777778e-3_f64 * t94227 * t103263 - 0.12356481481481481481e-2_f64 * t28392 * t28439;
    (t103263, t103268)
}
