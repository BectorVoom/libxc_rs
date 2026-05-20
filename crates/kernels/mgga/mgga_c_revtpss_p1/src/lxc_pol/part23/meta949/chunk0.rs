//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3137/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3137<F: Float>(t17529: F, t20786: F, t1042: F, t1247: F, t1250: F, t12956: F, t1715: F, t17505: F, t20809: F, t20876: F, t20880: F, t21242: F, t24759: F, t24773: F, t3708: F, t3711: F, t482: F, t5056: F, t5268: F, t5304: F, t58927: F, t6619: F, t69742: F, t82368: F, t82422: F) -> F {
    let t82434 = t17529 * t20786;
    let t82438 = F::cast_from(0.85748036236139473944e-3_f64) * t3711 * t1042 * t5268 * t82368 + F::cast_from(0.42874018118069736972e-3_f64) * t12956 * t24759 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t69742 * t1715 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t20809 * t5056 + F::cast_from(0.21437009059034868486e-3_f64) * t3708 * t24773 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t1042 * t482 * t82422 * t1250 - F::cast_from(0.45732285992607719436e-2_f64) * t58927 * t6619 - F::cast_from(0.45732285992607719436e-2_f64) * t17505 * t20880 - F::cast_from(0.45732285992607719436e-2_f64) * t17505 * t20876 + F::cast_from(0.22866142996303859718e-2_f64) * t82434 - F::cast_from(0.7622047665434619906e-2_f64) * t21242 * t5304;
    t82438
}
