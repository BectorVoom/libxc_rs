//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2881/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2881<F: Float>(t3006: F, t972: F, t4711: F, t52238: F, t11557: F, t15572: F, t981: F, t11452: F, t4669: F, t11404: F, t11410: F, t11444: F, t11450: F, t11521: F, t11548: F, t11554: F, t15238: F, t15242: F, t15249: F, t15252: F, t15255: F, t15274: F, t15283: F, t15284: F, t15413: F, t1621: F, t2944: F, t2962: F, t2968: F, t41662: F, t41740: F, t41742: F, t41775: F, t41785: F, t41788: F, t41799: F, t4652: F, t4673: F, t4674: F, t4690: F) -> (F, F, F, F) {
    let t52239 = t3006 * t972;
    let t52242 = F::cast_from(0.31168546390226634766e3_f64) * t52238 * t4711 * t52239;
    let t52245 = F::cast_from(0.14035736694323150897e2_f64) * t981 * t15572 * t11557;
    let t52264 = t4669 * t11452;
    let t52282 = -F::new(6.0) * t41775 * t4652 + F::cast_from(0.96491876992155210402e2_f64) * t41799 * t4674 - F::new(12.0) * t11548 * t15274 + F::cast_from(0.19298375398431042081e3_f64) * t11404 * t15284 + F::cast_from(0.96491876992155210402e2_f64) * t11404 * t15238 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t15283 * t2962 + F::cast_from(0.32163958997385070134e2_f64) * t2968 * t4673 * t11444 + F::cast_from(0.6207121550312808036e4_f64) * t41662 * t15242 + F::cast_from(0.6207121550312808036e4_f64) * t11450 * t52264 * t2944 + F::cast_from(0.19964560303604640732e6_f64) * t41740 * t1621 * t41742 * t11410 - F::cast_from(0.35089341735807877242e1_f64) * t15413 * t11521 - F::cast_from(0.35089341735807877242e1_f64) * t41785 * t4690 - F::cast_from(0.70178683471615754484e1_f64) * t11554 * t15249 - F::cast_from(0.35089341735807877242e1_f64) * t11554 * t15252 - F::cast_from(0.31168546390226634765e3_f64) * t41788 * t15255;
    (t52239, t52242, t52245, t52282)
}
