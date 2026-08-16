//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2756/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2756<F: Float>(t14652: F, t2661: F, t2662: F, t837: F, t2646: F, t4416: F, t14663: F, t221: F, t2484: F, t2485: F, t40691: F, t40696: F, t40700: F, t40705: F, t40707: F, t40711: F, t40719: F, t40722: F, t40728: F, t40732: F, t50722: F, t50724: F, t50728: F, t50732: F, t50736: F) -> F {
    let t50740 = t2661 * t2662 * t14652 * t837;
    let t50744 = t2661 * t2662 * t4416 * t2646;
    let t50748 = t2484 * t2485 * t221 * t14663;
    let t50750 = F::cast_from(0.33884236873090992594e-6_f64) * t40691 + F::cast_from(0.42874018118069736972e-4_f64) * t40696 - F::cast_from(0.42874018118069736972e-4_f64) * t40700 + F::cast_from(0.71456696863449561619e-5_f64) * t40705 - F::cast_from(0.68026775414003982663e-1_f64) * t40707 - F::cast_from(0.15415400852149882894e-1_f64) * t40711 - F::cast_from(0.50820002809285328224e-4_f64) * t40719 - F::cast_from(0.54885603034028154481e-3_f64) * t40722 - F::cast_from(0.76230004213927992337e-3_f64) * t40728 - F::cast_from(0.32524801797942610064e-2_f64) * t40732 - F::cast_from(0.24009450146119052704e-1_f64) * t50722 + F::cast_from(0.36014175219178579057e0_f64) * t50724 - F::cast_from(0.85748036236139473944e-4_f64) * t50728 + F::cast_from(0.42874018118069736972e-4_f64) * t50732 + F::cast_from(0.21437009059034868486e-4_f64) * t50736 - F::cast_from(0.17149607247227894789e-3_f64) * t50740 - F::cast_from(0.85748036236139473944e-4_f64) * t50744 - F::cast_from(0.38115002106963996168e-4_f64) * t50748;
    t50750
}
