//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1619/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1619(t1261: f64, t1264: f64, t12780: f64, t12800: f64, t12805: f64, t12822: f64, t12828: f64, t12832: f64, t12841: f64, t12846: f64, t12858: f64, t12866: f64, t12867: f64, t13055: f64, t13079: f64, t247: f64, t3630: f64, t3640: f64, t3644: f64, t3647: f64, t372: f64, t3720: f64, t43797: f64, t44484: f64, t44500: f64, t44501: f64, t44502: f64, t44508: f64, t44510: f64, t44517: f64, t44521: f64, t44526: f64) -> f64 {
    let t44529 = -0.25724410870841842184e-2_f64 * t12832 * t12805 - 0.51448821741683684368e-2_f64 * t44484 * t12858 - 0.85748036236139473944e-3_f64 * t12800 * t3640 - 0.57165357490759649296e-3_f64 * t3647 * t12822 - 0.14291339372689912324e-3_f64 * t1261 * t247 * t1264 * t43797 - 0.17149607247227894789e-2_f64 * t12800 * t3644 - 0.34299214494455789577e-2_f64 * t3647 * t12828 - 0.51448821741683684368e-2_f64 * t44500 * t3720 * t44501 * t44502 - 0.17149607247227894789e-2_f64 * t44508 + 0.34299214494455789578e-2_f64 * t44510 * t12867 * t12841 + 0.34299214494455789577e-2_f64 * t12866 * t12867 * t12780 - 0.17149607247227894789e-2_f64 * t44517 * t12867 * t12846 - 0.34299214494455789578e-2_f64 * t44521 * t372 * t13079 * t3630 - 0.51448821741683684368e-2_f64 * t44526 * t13055;
    t44529
}
