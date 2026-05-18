//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1358/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1358<F: Float>(t1444: F, t1943: F, t5654: F, t98359: F, t102011: F, t102014: F, t102017: F, t103083: F, t103224: F, t103240: F, t103251: F, t103255: F, t103258: F, t28388: F, t28392: F, t28439: F, t3984: F, t59401: F, t7908: F, t7909: F, t94227: F) -> (F, F) {
    let t103263 = t98359 * t1943 * t1444 * t5654;
    let t103268 = F::new(0.185671721767578125e-4) * t28388 * t103224 + F::new(0.111403033060546875e-3) * t28388 * t103083 + F::new(0.20612155671296296296e-4) * t103240 - F::new(0.33163888888888888888e-2) * t102011 - F::new(0.33163888888888888888e-2) * t102014 + F::new(0.66327777777777777776e-2) * t102017 + F::new(0.23168402777777777778e-3) * t7908 * t3984 * t7909 * t59401 + F::new(0.23168402777777777778e-3) * t7908 * t103251 + F::new(0.15445601851851851852e-3) * t103255 - F::new(0.61836467013888888889e-4) * t94227 * t103258 - F::new(0.12367293402777777778e-3) * t94227 * t103263 - F::new(0.12356481481481481481e-2) * t28392 * t28439;
    (t103263, t103268)
}
