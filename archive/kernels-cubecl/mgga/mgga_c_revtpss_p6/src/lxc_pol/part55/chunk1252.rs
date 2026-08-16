//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1252/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1252<F: Float>(t125609: F, t8477: F, t1955: F, t32715: F, t128617: F, t1426: F, t7286: F, t786: F, t122391: F, t122393: F, t122399: F, t125717: F, t125721: F, t125732: F, t128618: F, t26304: F, t27840: F, t27869: F, t28012: F, t32720: F, t46361: F, t7304: F, t8702: F, t8705: F) -> (F, F, F) {
    let t128691 = t8477 * t125609;
    let t128694 = t1955 * t32715;
    let t128709 = t128617 * t1426;
    let t128711 = t786 * t128709 * t7286;
    let t128713 = F::cast_from(0.8673628188205199462e0_f64) * t128618 * t7304 - F::cast_from(0.11423947533020470523e1_f64) * t128691 * t32720 + F::cast_from(0.8673628188205199462e0_f64) * t128694 * t27869 + F::cast_from(0.6854368519812282314e1_f64) * t8477 * t8705 * t46361 * t26304 * t27840 - F::cast_from(0.22312397525430606492e-2_f64) * t125717 - F::cast_from(0.7437465841810202164e-2_f64) * t125721 - F::cast_from(0.69416347856895220197e-2_f64) * t125732 - F::cast_from(0.8673628188205199462e0_f64) * t8702 * t28012 + F::cast_from(0.14456046980341999104e-1_f64) * t122391 - F::cast_from(0.25702851531048074406e-1_f64) * t122393 + F::cast_from(0.14456046980341999104e-1_f64) * t128711 + t122399;
    (t128694, t128709, t128713)
}
