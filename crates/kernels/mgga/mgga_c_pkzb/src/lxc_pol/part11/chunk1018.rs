//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1018/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1018<F: Float>(t11213: F, t890: F, t11180: F, t6233: F, t11166: F, t871: F, t3740: F, t8009: F, t11159: F, t11164: F, t11167: F, t11181: F, t1197: F, t1209: F, t2279: F, t2318: F, t3083: F, t3116: F, t3780: F, t3793: F, t3807: F, t3820: F, t3823: F, t6282: F, t6288: F, t6323: F, t8071: F, t8107: F, t8211: F, t882: F, t9891: F, t9964: F) -> (F, F, F, F, F, F) {
    let t11214 = t11213 * t890;
    let t11217 = t11180 * t6233;
    let t11222 = t11166 * t871;
    let t11227 = t11180 * t890;
    let t11231 = F::new(6.0) * t8009 * t3740;
    let t11232 = t11159 - F::new(0.19751673498613801407e-1) * t11164 + F::new(0.2069040516770936012e4) * t6288 * t11167 + F::new(3.0) * t9891 * t1197 + F::new(3.0) * t3083 * t3793 + F::new(0.17544670867903938621e1) * t9964 * t1209 + F::new(0.17544670867903938621e1) * t3116 * t3820 + F::new(0.51947577317044391276e2) * t8107 * t3823 - F::new(0.10389515463408878255e3) * t6323 * t11181 + F::new(0.5848223622634646207e0) * t882 * t11214 + F::new(0.10254018858216406658e4) * t6282 * t11217 - F::new(6.0) * t8211 * t3780 + F::new(6.0) * t2279 * t11222 - F::new(0.35089341735807877242e1) * t8071 * t3807 + F::new(0.35089341735807877242e1) * t2318 * t11227 + t11231;
    (t11214, t11217, t11222, t11227, t11231, t11232)
}
