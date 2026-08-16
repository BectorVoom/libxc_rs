//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2231/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2231<F: Float>(t11409: F, t11450: F, t11466: F, t15350: F, t15406: F, t1634: F, t19156: F, t23665: F, t23723: F, t23755: F, t23758: F, t23761: F, t23764: F, t23769: F, t23772: F, t23773: F, t23776: F, t23785: F, t23798: F, t23812: F, t2943: F, t2968: F, t2987: F, t3012: F, t311: F, t4685: F, t6177: F, t6206: F, t6209: F, t946: F) -> F {
    let t23814 = F::cast_from(0.96491876992155210402e2_f64) * t15406 * t6177 - F::cast_from(0.19298375398431042081e3_f64) * t11409 * t23723 + F::cast_from(1.0_f64) * t946 * t23755 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t23758 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t23761 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t23764 + t23769 - t23772 - F::cast_from(6.0_f64) * t2943 * t23773 + F::cast_from(0.2069040516770936012e4_f64) * t11450 * t23776 + F::cast_from(0.17544670867903938621e1_f64) * t19156 * t1634 + F::cast_from(0.17544670867903938621e1_f64) * t4685 * t6206 + F::cast_from(0.51947577317044391276e2_f64) * t15350 * t6209 - F::cast_from(0.10389515463408878255e3_f64) * t11466 * t23785 - F::cast_from(0.310907e-1_f64) * t23798 * t311 - F::cast_from(0.19751673498613801407e-1_f64) * t23812 + t23665;
    t23814
}
