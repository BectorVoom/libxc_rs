//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1238/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1238<F: Float>(t10792: F, t10795: F, t10842: F, t10878: F, t1095: F, t1096: F, t1916: F, t1938: F, t26053: F, t26062: F, t26336: F, t2815: F, t2816: F, t2834: F, t2853: F, t30221: F, t30223: F, t30225: F, t30227: F, t3565: F, t3577: F, t5825: F, t5830: F, t5871: F, t5897: F, t702: F, t9422: F, t9493: F) -> F {
    let t30440 = t30221 - t30223 + t30225 - t30227 - F::new(24.0) * t5830 * t10878 * t702 + F::new(18.0) * t1938 * t3565 * t2815 + F::new(0.11579025239058625248e4) * t5871 * t10842 * t702 - F::new(6.0) * t5897 * t10792 - F::new(6.0) * t1916 * t2816 * t3577 - F::new(6.0) * t1916 * t1096 * t9493 + F::new(0.96491876992155210402e2) * t5825 * t10795 + F::new(0.96491876992155210402e2) * t1938 * t26336 * t1095 + F::new(0.96491876992155210402e2) * t1938 * t9422 * t2815 - F::new(0.35089341735807877242e1) * t26062 * t2834 + F::new(0.51947577317044391276e2) * t26053 * t2853;
    t30440
}
