//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1619/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1619<F: Float>(t1261: F, t1264: F, t12780: F, t12800: F, t12805: F, t12822: F, t12828: F, t12832: F, t12841: F, t12846: F, t12858: F, t12866: F, t12867: F, t13055: F, t13079: F, t247: F, t3630: F, t3640: F, t3644: F, t3647: F, t372: F, t3720: F, t43797: F, t44484: F, t44500: F, t44501: F, t44502: F, t44508: F, t44510: F, t44517: F, t44521: F, t44526: F) -> F {
    let t44529 = -F::cast_from(0.25724410870841842184e-2_f64) * t12832 * t12805 - F::cast_from(0.51448821741683684368e-2_f64) * t44484 * t12858 - F::cast_from(0.85748036236139473944e-3_f64) * t12800 * t3640 - F::cast_from(0.57165357490759649296e-3_f64) * t3647 * t12822 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t247 * t1264 * t43797 - F::cast_from(0.17149607247227894789e-2_f64) * t12800 * t3644 - F::cast_from(0.34299214494455789577e-2_f64) * t3647 * t12828 - F::cast_from(0.51448821741683684368e-2_f64) * t44500 * t3720 * t44501 * t44502 - F::cast_from(0.17149607247227894789e-2_f64) * t44508 + F::cast_from(0.34299214494455789578e-2_f64) * t44510 * t12867 * t12841 + F::cast_from(0.34299214494455789577e-2_f64) * t12866 * t12867 * t12780 - F::cast_from(0.17149607247227894789e-2_f64) * t44517 * t12867 * t12846 - F::cast_from(0.34299214494455789578e-2_f64) * t44521 * t372 * t13079 * t3630 - F::cast_from(0.51448821741683684368e-2_f64) * t44526 * t13055;
    t44529
}
