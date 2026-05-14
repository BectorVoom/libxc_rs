//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 807/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk807<F: Float>(t22813: F, t828: F, t9942: F, t1414: F, t22809: F, t22079: F, t3936: F, t6869: F, t13790: F, t5673: F, t1883: F, t22074: F, t13765: F, t13779: F, t13781: F, t1410: F, t22023: F, t22028: F, t22030: F, t3934: F, t5671: F, t9711: F, t9725: F, t9729: F) -> (F, F, F, F, F, F) {
    let t22815 = t9942 * t828 * t22813;
    let t22822 = t1414 * t828 * t22809;
    let t22829 = t3936 * t22079 * t6869;
    let t22833 = t5673 * t22079 * t13790;
    let t22837 = t3936 * t22074 * t1883;
    let t22840 = -0.25724410870841842183e-1 * t1410 * t22815 + 0.21437009059034868486e-4 * t22023 - 0.42874018118069736972e-4 * t22028 + 0.12004725073059526352e-1 * t22030 + t9711 + t9725 - t9729 - 0.85748036236139473944e-3 * t1410 * t22822 + 0.16262400898971305032e-2 * t13765 - 0.22866142996303859718e-3 * t13779 - 0.68026775414003982663e-1 * t13781 + 0.25724410870841842183e-2 * t3934 * t22829 + 0.12862205435420921092e-2 * t5671 * t22833 + 0.25724410870841842183e-2 * t3934 * t22837;
    (t22815, t22822, t22829, t22833, t22837, t22840)
}
