//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1213/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1213<F: Float>(t2815: F, t5873: F, t1954: F, t2826: F, t1100: F, t5498: F, t1976: F, t20716: F, t17351: F, t17354: F, t17357: F, t17601: F, t17664: F, t1911: F, t1917: F, t1932: F, t1938: F, t1940: F, t1957: F, t1972: F, t1980: F, t20705: F, t20719: F, t20745: F, t20820: F, t248: F, t2819: F, t2829: F, t5825: F, t5831: F, t5839: F, t5842: F, t5865: F, t5871: F, t702: F, t714: F, t722: F, t7240: F, t7241: F, t7244: F, t7247: F, t7407: F, t7408: F, t7478: F) -> F {
    let t21134 = t2815 * t5873;
    let t21143 = t2826 * t1954;
    let t21146 = t1100 * t5498;
    let t21156 = t2826 * t1976;
    let t21165 = F::new(0.68493333333333333332e-1) * t20716;
    let t21171 = -F::new(0.24828486201251232145e5) * t17601 * t7247 * t5831 + F::new(0.19298375398431042081e3) * t5825 * t7241 + F::new(0.96491876992155210402e2) * t1938 * t7407 * t1940 * t702 + F::new(0.96491876992155210402e2) * t1938 * t7240 * t1932 + F::new(0.6207121550312808036e4) * t5871 * t21134 * t1917 + F::new(0.96491876992155210402e2) * t5825 * t7244 + F::new(0.32163958997385070134e2) * t1938 * t2819 * t5865 - F::new(0.35089341735807877242e1) * t21143 * t1957 - F::new(0.10389515463408878255e3) * t21146 * t5839 + F::new(3.0) * t1911 * t7408 + F::new(0.5848223622634646207e0) * t714 * t20820 * t722 + F::new(0.17544670867903938621e1) * t7478 * t1972 + F::new(0.51947577317044391276e2) * t21156 * t1980 + F::new(0.5848223622634646207e0) * t2829 * t5842 - F::new(0.310907e-1) * (t17664 - F::new(0.15981777777777777777e0) * t17351 + F::new(0.68493333333333333333e-1) * t17354 - F::new(0.17123333333333333333e-1) * t17357 - F::new(0.53272592592592592592e-1) * t20705 + t21165 - F::new(0.51369999999999999999e-1) * t20719 + F::new(0.5137e-1) * t20745) * t248;
    t21171
}
