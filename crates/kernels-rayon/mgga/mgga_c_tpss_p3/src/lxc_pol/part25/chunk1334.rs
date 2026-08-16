//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1334/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1334(t1163: f64, t1273: f64, t1322: f64, t13627: f64, t13856: f64, t13955: f64, t1760: f64, t1846: f64, t18547: f64, t18690: f64, t19579: f64, t19620: f64, t20289: f64, t20379: f64, t2056: f64, t20640: f64, t21253: f64, t21897: f64, t21907: f64, t21944: f64, t3491: f64, t3499: f64, t3537: f64, t3538: f64, t4674: f64, t51622: f64, t5314: f64, t5815: f64, t5895: f64, t5910: f64, t5936: f64, t5937: f64, t6103: f64, t626: f64, t6399: f64, t67246: f64, t68827: f64, t68975: f64, t7383: f64) -> f64 {
    let t71303 = -2.0_f64 * t626 * t1163 * t21907 - t1760 * t5936 * t13955 + t68975 * t1846 + t21253 * t5937 - 3.0_f64 * t18547 * t18690 * t51622 + 6.0_f64 * t19620 * t7383 * t13856 + 2.0_f64 * t1760 * t5936 * t13627 - 2.0_f64 * t3491 * t6399 - 2.0_f64 * t1322 * t20640 - 4.0_f64 * t20289 * t3538 - 2.0_f64 * t626 * t5314 * t5815 - 2.0_f64 * t2056 * t21897 - 2.0_f64 * t3499 * t21897 - 2.0_f64 * t626 * t5895 * t4674 - 4.0_f64 * t6103 * t20379 - 4.0_f64 * t626 * t6399 * t3537 + 3.0_f64 * t21253 * t5910 + t21944 * t1273 - 6.0_f64 * t19579 * t67246 * t68827;
    t71303
}
