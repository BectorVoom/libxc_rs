//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2910/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2910<F: Float>(t291: F, t52782: F, t52803: F, t11531: F, t15421: F, t2942: F, t4644: F, t11408: F, t1614: F, t11411: F, t11502: F, t11510: F, t15343: F, t1634: F, t2945: F, t3007: F, t3015: F, t41746: F, t4685: F, t52522: F, t52536: F, t52549: F, t52562: F, t52574: F, t52588: F, t52601: F, t52615: F, t52628: F, t52637: F, t52642: F, t52647: F, t52650: F, t52652: F, t52762: F, t946: F, t954: F, t974: F) -> (F, F, F) {
    let t52806 = F::cast_from(0.621814e-1_f64) * (t52782 + t52803) * t291;
    let t52808 = F::cast_from(6.0_f64) * t15421 * t11531;
    let t52809 = t4644 * t2942;
    let t52812 = t1614 * t11408;
    let t52817 = F::cast_from(0.17544670867903938621e1_f64) * t52522 * t974 + F::cast_from(1.0_f64) * t946 * (t52536 + t52549 + t52562 + t52574 + t52588 + t52601 + t52615 + t52628) * t954 + F::cast_from(0.17544670867903938621e1_f64) * t15343 * t3007 + F::cast_from(0.51947577317044391276e2_f64) * t52637 * t3015 + F::cast_from(0.5848223622634646207e0_f64) * t4685 * t11502 + F::cast_from(0.10254018858216406658e4_f64) * t52642 * t11510 - t52647 - t52650 - t52652 - t52762 + t52806 - t52808 - F::cast_from(6.0_f64) * t52809 * t2945 - F::cast_from(0.19298375398431042081e3_f64) * t52812 * t11411 + F::cast_from(0.5848223622634646207e0_f64) * t41746 * t1634;
    (t52806, t52808, t52817)
}
