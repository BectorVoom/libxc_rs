//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2642/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2642<F: Float>(t125: F, t13920: F, t13955: F, t46946: F, t13775: F, t808: F, t9845: F, t13783: F, t13784: F, t13789: F, t13926: F, t13944: F, t1399: F, t36776: F, t3934: F, t3936: F, t3938: F, t46671: F, t46680: F, t48475: F, t48553: F, t48557: F, t48563: F, t48565: F, t48573: F, t48577: F, t48591: F, t48593: F, t9810: F) -> (F, F) {
    let t48595 = t125 * t13920;
    let t48600 = t46946 * t13955;
    let t48603 = t9845 * t808 * t13775;
    let t48604 = F::cast_from(0.76230004213927992336e-5_f64) * t48603;
    let t48611 = -F::cast_from(0.17149607247227894789e-3_f64) * t48553 - F::cast_from(0.85748036236139473944e-4_f64) * t48557 - F::cast_from(0.20082057720118594944e-6_f64) * t48563 - F::cast_from(0.60023625365297631762e-1_f64) * t48565 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t13784 * t9810 + F::cast_from(0.12862205435420921092e-3_f64) * t48573 - F::cast_from(0.12862205435420921092e-3_f64) * t48577 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t36776 * t48475 * t1399 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t48475 * t3938 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t13926 * t9810 + F::cast_from(0.30011812682648815881e-2_f64) * t48591 + F::cast_from(0.24009450146119052704e-1_f64) * t48593 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t48595 * t3938 - F::cast_from(0.45738002528356795401e-4_f64) * t48600 + t48604 - F::cast_from(0.54885603034028154481e-3_f64) * t46671 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t13944 * t9810 + F::cast_from(0.15246000842785598467e-3_f64) * t46680;
    (t48595, t48611)
}
