//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3153/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3153<F: Float>(t17633: F, t471: F, t24770: F, t3153: F, t12784: F, t17605: F, t20272: F, t21022: F, t21228: F, t24792: F, t24794: F, t24798: F, t3625: F, t3626: F, t3720: F, t5340: F, t5341: F, t5402: F, t6425: F, t69885: F, t69890: F, t70995: F, t71275: F) -> (F, F, F) {
    let t82838 = t17633 * t471;
    let t82859 = t24770 * t3153;
    let t82864 = -F::cast_from(0.85748036236139473944e-3_f64) * t12784 * t24798 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t3626 * t6425 * t82838 - F::cast_from(0.14481890564325777821e-1_f64) * t70995 * t5402 + F::cast_from(0.45732285992607719436e-2_f64) * t71275 * t5402 + F::cast_from(0.45732285992607719436e-2_f64) * t17605 * t21022 + F::cast_from(0.45732285992607719436e-2_f64) * t17605 * t21228 - F::cast_from(0.42874018118069736972e-3_f64) * t12784 * t24794 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t20272 * t24792 + F::cast_from(0.57165357490759649295e-3_f64) * t69885 - F::cast_from(0.47637797908966374413e-3_f64) * t69890 + F::cast_from(0.42874018118069736972e-3_f64) * t5340 * t3720 * t82859 * t5341;
    (t82838, t82859, t82864)
}
