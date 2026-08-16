//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2875/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2875<F: Float>(t213: F, t23359: F, t1580: F, t18663: F, t18785: F, t18800: F, t225: F, t23413: F, t257: F, t41078: F, t41118: F, t4474: F, t4534: F, t51733: F, t51742: F, t51756: F, t63085: F, t63091: F, t63094: F, t63099: F, t63103: F, t63109: F, t77151: F, t865: F, t886: F, t887: F) -> F {
    let t77316 = t213 * t23359;
    let t77326 = -F::cast_from(0.19756347548806534796e1_f64) * t18800 * t4534 + F::cast_from(0.78059524315062264151e-2_f64) * t51733 - F::cast_from(0.19756347548806534796e1_f64) * t63103 * t1580 - F::cast_from(0.29272321618148349057e-1_f64) * t63085 + t51742 - F::cast_from(0.65854491829355115984e-1_f64) * t63091 + F::cast_from(0.11708928647259339623e0_f64) * t63094 + F::cast_from(0.7805952431506226415e-1_f64) * t63099 + F::cast_from(0.15805078039045227836e2_f64) * t865 * t41078 * t23413 * t886 - F::cast_from(0.11853808529283920877e2_f64) * t4474 * t18663 - F::cast_from(0.19756347548806534796e1_f64) * t4474 * t18785 - F::cast_from(0.65854491829355115987e0_f64) * t77316 * t887 - F::cast_from(0.58544643236296698113e-1_f64) * t63109 + F::cast_from(0.11044544084478153697e-3_f64) * t41118 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t77151 * t225 * t257 - F::cast_from(0.39029762157531132076e-2_f64) * t51756;
    t77326
}
