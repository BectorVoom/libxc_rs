//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3708/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3708<F: Float>(t12916: F, t17709: F, t20958: F, t1012: F, t1122: F, t1222: F, t1238: F, t17280: F, t17290: F, t17711: F, t1791: F, t20747: F, t20956: F, t3601: F, t3626: F, t3699: F, t3720: F, t44535: F, t44586: F, t5320: F, t5327: F, t57045: F, t57049: F, t57265: F, t58920: F, t59001: F, t59033: F, t60717: F, t70221: F, t70225: F, t70235: F) -> F {
    let t70250 = t17709 * t12916 * t20958;
    let t70254 = -F::cast_from(0.42874018118069736972e-3_f64) * t59033 * t1791 - F::cast_from(0.85748036236139473944e-3_f64) * t17290 * t5320 - F::cast_from(0.42874018118069736972e-3_f64) * t5327 * t17280 + F::cast_from(0.45732285992607719436e-2_f64) * t70221 * t1238 - t70225 / F::cast_from(972.0_f64) + t1222 * t1012 * t3699 * t60717 / F::cast_from(108.0_f64) + F::cast_from(0.12862205435420921092e-2_f64) * t17709 * t3720 * t20956 * t44586 + F::cast_from(0.51448821741683684368e-2_f64) * t58920 * t3720 * t70235 * t44535 * t3601 - F::cast_from(0.77173232612525526552e-2_f64) * t59001 * t3720 * t70235 * t17711 + F::cast_from(0.17149607247227894789e-2_f64) * t57265 * t3626 * t20747 * t1122 + F::cast_from(0.17149607247227894789e-2_f64) * t70250 - F::cast_from(0.57165357490759649296e-3_f64) * t57045 + F::cast_from(0.30488190661738479624e-2_f64) * t57049;
    t70254
}
