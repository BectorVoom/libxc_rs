//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1212/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1212<F: Float>(t1915: F, t2793: F, t1088: F, t5829: F, t1107: F, t1977: F, t1096: F, t1108: F, t17391: F, t1916: F, t1917: F, t1918: F, t1932: F, t21062: F, t21082: F, t2816: F, t5484: F, t5830: F, t5832: F, t5838: F, t5865: F, t5887: F, t5890: F, t5894: F, t5897: F, t5906: F, t695: F, t702: F, t703: F, t7231: F, t7234: F, t7237: F, t7240: F, t7315: F, t7324: F, t7408: F) -> F {
    let t21087 = t2793 * t1915;
    let t21090 = t1088 * t5829;
    let t21093 = t1977 * t1107;
    let t21121 = F::new(0.35089341735807877242e1) * t7315 * t5906 + F::new(1.0) * t695 * (t21062 + t21082) * t703 - F::new(6.0) * t21087 * t1918 - F::new(0.19298375398431042081e3) * t21090 * t5832 + F::new(0.10526802520742363173e2) * t21093 * t5890 + F::new(0.51947577317044391277e2) * t7315 * t5894 + F::new(0.96491876992155210402e2) * t7324 * t5887 - F::new(0.14035736694323150897e2) * t5838 * t1108 * t5484 - F::new(12.0) * t5897 * t7231 - F::new(6.0) * t1916 * t7408 * t702 - F::new(6.0) * t1916 * t2816 * t1932 - F::new(0.57895126195293126242e3) * t5830 * t7240 * t1917 - F::new(6.0) * t5897 * t7234 - F::new(2.0) * t1916 * t1096 * t5865 - F::new(0.57895126195293126242e3) * t17391 * t7237;
    t21121
}
