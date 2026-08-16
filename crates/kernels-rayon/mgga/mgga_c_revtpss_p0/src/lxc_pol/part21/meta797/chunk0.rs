//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2881/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2881(t3006: f64, t972: f64, t4711: f64, t52238: f64, t11557: f64, t15572: f64, t981: f64, t11452: f64, t4669: f64, t11404: f64, t11410: f64, t11444: f64, t11450: f64, t11521: f64, t11548: f64, t11554: f64, t15238: f64, t15242: f64, t15249: f64, t15252: f64, t15255: f64, t15274: f64, t15283: f64, t15284: f64, t15413: f64, t1621: f64, t2944: f64, t2962: f64, t2968: f64, t41662: f64, t41740: f64, t41742: f64, t41775: f64, t41785: f64, t41788: f64, t41799: f64, t4652: f64, t4673: f64, t4674: f64, t4690: f64) -> (f64, f64, f64, f64) {
    let t52239 = t3006 * t972;
    let t52242 = 0.31168546390226634766e3_f64 * t52238 * t4711 * t52239;
    let t52245 = 0.14035736694323150897e2_f64 * t981 * t15572 * t11557;
    let t52264 = t4669 * t11452;
    let t52282 = -6.0_f64 * t41775 * t4652 + 0.96491876992155210402e2_f64 * t41799 * t4674 - 12.0_f64 * t11548 * t15274 + 0.19298375398431042081e3_f64 * t11404 * t15284 + 0.96491876992155210402e2_f64 * t11404 * t15238 + 0.96491876992155210402e2_f64 * t2968 * t15283 * t2962 + 0.32163958997385070134e2_f64 * t2968 * t4673 * t11444 + 0.6207121550312808036e4_f64 * t41662 * t15242 + 0.6207121550312808036e4_f64 * t11450 * t52264 * t2944 + 0.19964560303604640732e6_f64 * t41740 * t1621 * t41742 * t11410 - 0.35089341735807877242e1_f64 * t15413 * t11521 - 0.35089341735807877242e1_f64 * t41785 * t4690 - 0.70178683471615754484e1_f64 * t11554 * t15249 - 0.35089341735807877242e1_f64 * t11554 * t15252 - 0.31168546390226634765e3_f64 * t41788 * t15255;
    (t52239, t52242, t52245, t52282)
}
