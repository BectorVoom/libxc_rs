//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1688/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1688<F: Float>(t6305: F, t373: F, t6299: F, t1042: F, t1063: F, t1066: F, t11875: F, t15707: F, t15716: F, t1592: F, t23844: F, t23848: F, t23852: F, t247: F, t3117: F, t3127: F, t3150: F, t3155: F, t3162: F, t42868: F, t42873: F, t42984: F, t42985: F, t4834: F, t6263: F, t6271: F, t65292: F, t65717: F, t78512: F, t78550: F, t78607: F, t79301: F, t88083: F) -> (F, F, F, F, F) {
    let t88694 = t6305 * t6305;
    let t88695 = t373 * t88694;
    let t88714 = t6299 * t6299;
    let t88715 = t373 * t88714;
    let t88727 = F::cast_from(0.25724410870841842184e-2_f64) * t11875 * t3117 * t6271 * t3162 * t6299 - F::cast_from(0.34299214494455789578e-2_f64) * t1063 * t247 * t1066 * t88083 - F::cast_from(0.31758531939310916276e-3_f64) * t65292 - F::cast_from(0.34299214494455789577e-2_f64) * t78512 + F::cast_from(0.51448821741683684368e-2_f64) * t42868 * t1042 * t88695 * t42873 - F::cast_from(0.17149607247227894789e-2_f64) * t65717 * t6263 - F::cast_from(0.34299214494455789578e-2_f64) * t15716 * t1042 * t78607 * t1592 - F::cast_from(0.57165357490759649296e-3_f64) * t3127 * t1042 * t79301 * t1592 - F::cast_from(0.34299214494455789578e-2_f64) * t4834 * t23852 + F::cast_from(0.28582678745379824648e-2_f64) * t4834 * t23844 + F::cast_from(0.12862205435420921092e-2_f64) * t3150 * t1042 * t88715 * t3155 - F::cast_from(0.28582678745379824648e-2_f64) * t15707 * t23848 + F::cast_from(0.30011812682648815881e-2_f64) * t42984 * t1042 * t88695 * t42985 - F::cast_from(0.3811023832717309953e-2_f64) * t78550;
    (t88694, t88695, t88714, t88715, t88727)
}
