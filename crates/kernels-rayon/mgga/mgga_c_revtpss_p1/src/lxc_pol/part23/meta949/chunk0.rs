//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3137/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3137(t17529: f64, t20786: f64, t1042: f64, t1247: f64, t1250: f64, t12956: f64, t1715: f64, t17505: f64, t20809: f64, t20876: f64, t20880: f64, t21242: f64, t24759: f64, t24773: f64, t3708: f64, t3711: f64, t482: f64, t5056: f64, t5268: f64, t5304: f64, t58927: f64, t6619: f64, t69742: f64, t82368: f64, t82422: f64) -> f64 {
    let t82434 = t17529 * t20786;
    let t82438 = 0.85748036236139473944e-3_f64 * t3711 * t1042 * t5268 * t82368 + 0.42874018118069736972e-3_f64 * t12956 * t24759 + 0.42874018118069736972e-3_f64 * t3711 * t1042 * t69742 * t1715 + 0.42874018118069736972e-3_f64 * t3711 * t1042 * t20809 * t5056 + 0.21437009059034868486e-3_f64 * t3708 * t24773 + 0.21437009059034868486e-3_f64 * t1247 * t1042 * t482 * t82422 * t1250 - 0.45732285992607719436e-2_f64 * t58927 * t6619 - 0.45732285992607719436e-2_f64 * t17505 * t20880 - 0.45732285992607719436e-2_f64 * t17505 * t20876 + 0.22866142996303859718e-2_f64 * t82434 - 0.7622047665434619906e-2_f64 * t21242 * t5304;
    t82438
}
