//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3153/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3153(t17633: f64, t471: f64, t24770: f64, t3153: f64, t12784: f64, t17605: f64, t20272: f64, t21022: f64, t21228: f64, t24792: f64, t24794: f64, t24798: f64, t3625: f64, t3626: f64, t3720: f64, t5340: f64, t5341: f64, t5402: f64, t6425: f64, t69885: f64, t69890: f64, t70995: f64, t71275: f64) -> (f64, f64, f64) {
    let t82838 = t17633 * t471;
    let t82859 = t24770 * t3153;
    let t82864 = -0.85748036236139473944e-3_f64 * t12784 * t24798 - 0.85748036236139473944e-3_f64 * t3625 * t3626 * t6425 * t82838 - 0.14481890564325777821e-1_f64 * t70995 * t5402 + 0.45732285992607719436e-2_f64 * t71275 * t5402 + 0.45732285992607719436e-2_f64 * t17605 * t21022 + 0.45732285992607719436e-2_f64 * t17605 * t21228 - 0.42874018118069736972e-3_f64 * t12784 * t24794 - 0.42874018118069736972e-3_f64 * t3625 * t3626 * t20272 * t24792 + 0.57165357490759649295e-3_f64 * t69885 - 0.47637797908966374413e-3_f64 * t69890 + 0.42874018118069736972e-3_f64 * t5340 * t3720 * t82859 * t5341;
    (t82838, t82859, t82864)
}
