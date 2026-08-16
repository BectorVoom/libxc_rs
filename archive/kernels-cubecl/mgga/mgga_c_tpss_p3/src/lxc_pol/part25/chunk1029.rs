//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1029/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1029<F: Float>(t14245: F, t2389: F, t774: F, t10617: F, t10620: F, t10630: F, t10635: F, t10642: F, t14220: F, t14223: F, t14229: F, t14234: F, t14238: F, t14242: F, t2173: F, t3626: F, t797: F, t8131: F) -> (F, F) {
    let t14247 = t2389 * t774 * t14245;
    let t14250 = -F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14220 + t3626 * t14223 / F::cast_from(1536.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t10617 + t10620 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t8131 + t2173 * t14229 / F::cast_from(384.0_f64) + t2173 * t14234 / F::cast_from(384.0_f64) + t10630 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t10635 - t10642 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t14238 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t797 * t14242 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t797 * t14247;
    (t14247, t14250)
}
