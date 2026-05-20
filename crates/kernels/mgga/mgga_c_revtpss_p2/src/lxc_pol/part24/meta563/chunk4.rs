//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1699/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1699<F: Float>(t88980: F, t88981: F, t88983: F, t88996: F, t1041: F, t1042: F, t1045: F, t1592: F, t16067: F, t16089: F, t16199: F, t19450: F, t19968: F, t23830: F, t23839: F, t23929: F, t23934: F, t23964: F, t3092: F, t3117: F, t3127: F, t357: F, t373: F, t42121: F, t43291: F, t4801: F, t4892: F, t4899: F, t54500: F, t54564: F, t6299: F, t6308: F, t6331: F, t65339: F, t78873: F, t79071: F, t88901: F, t88925: F, t88948: F) -> (F, F) {
    let t88998 = t88980 + t88981 + t88983 + t88996;
    let t89009 = -F::cast_from(0.17149607247227894789e-2_f64) * t19968 * t6331 - F::cast_from(0.51448821741683684368e-2_f64) * t43291 * t3117 * t88948 * t1045 + F::cast_from(0.34299214494455789578e-2_f64) * t16089 * t3092 * t23964 * t1592 + F::cast_from(0.12862205435420921092e-2_f64) * t16067 * t3117 * t19450 * t357 * t6299 + F::cast_from(0.17149607247227894789e-2_f64) * t4892 * t3117 * t78873 * t23929 - F::cast_from(0.85748036236139473944e-3_f64) * t4899 * t3117 * t78873 * t23934 + F::cast_from(0.51448821741683684368e-2_f64) * t54500 * t23839 + F::cast_from(0.17149607247227894789e-2_f64) * t3127 * t1042 * t4801 * t88901 + F::cast_from(0.57165357490759649296e-2_f64) * t3127 * t1042 * t16199 * t88925 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t1042 * t373 * t88998 * t1045 + F::cast_from(0.25724410870841842184e-2_f64) * t65339 * t6308 + F::cast_from(0.51448821741683684368e-2_f64) * t54564 * t23830 - t42121 + F::cast_from(0.17149607247227894789e-2_f64) * t79071;
    (t88998, t89009)
}
