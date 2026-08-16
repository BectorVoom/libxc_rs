//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1688/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1688(t6305: f64, t373: f64, t6299: f64, t1042: f64, t1063: f64, t1066: f64, t11875: f64, t15707: f64, t15716: f64, t1592: f64, t23844: f64, t23848: f64, t23852: f64, t247: f64, t3117: f64, t3127: f64, t3150: f64, t3155: f64, t3162: f64, t42868: f64, t42873: f64, t42984: f64, t42985: f64, t4834: f64, t6263: f64, t6271: f64, t65292: f64, t65717: f64, t78512: f64, t78550: f64, t78607: f64, t79301: f64, t88083: f64) -> (f64, f64, f64, f64, f64) {
    let t88694 = t6305 * t6305;
    let t88695 = t373 * t88694;
    let t88714 = t6299 * t6299;
    let t88715 = t373 * t88714;
    let t88727 = 0.25724410870841842184e-2_f64 * t11875 * t3117 * t6271 * t3162 * t6299 - 0.34299214494455789578e-2_f64 * t1063 * t247 * t1066 * t88083 - 0.31758531939310916276e-3_f64 * t65292 - 0.34299214494455789577e-2_f64 * t78512 + 0.51448821741683684368e-2_f64 * t42868 * t1042 * t88695 * t42873 - 0.17149607247227894789e-2_f64 * t65717 * t6263 - 0.34299214494455789578e-2_f64 * t15716 * t1042 * t78607 * t1592 - 0.57165357490759649296e-3_f64 * t3127 * t1042 * t79301 * t1592 - 0.34299214494455789578e-2_f64 * t4834 * t23852 + 0.28582678745379824648e-2_f64 * t4834 * t23844 + 0.12862205435420921092e-2_f64 * t3150 * t1042 * t88715 * t3155 - 0.28582678745379824648e-2_f64 * t15707 * t23848 + 0.30011812682648815881e-2_f64 * t42984 * t1042 * t88695 * t42985 - 0.3811023832717309953e-2_f64 * t78550;
    (t88694, t88695, t88714, t88715, t88727)
}
