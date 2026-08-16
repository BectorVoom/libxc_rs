//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1125/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1125(t13765: f64, t13779: f64, t13781: f64, t1410: f64, t22023: f64, t22028: f64, t22030: f64, t22815: f64, t22822: f64, t22829: f64, t22833: f64, t22837: f64, t3934: f64, t5671: f64, t9711: f64, t9725: f64, t9729: f64) -> f64 {
    let t22840 = -0.25724410870841842183e-1_f64 * t1410 * t22815 + 0.21437009059034868486e-4_f64 * t22023 - 0.42874018118069736972e-4_f64 * t22028 + 0.12004725073059526352e-1_f64 * t22030 + t9711 + t9725 - t9729 - 0.85748036236139473944e-3_f64 * t1410 * t22822 + 0.16262400898971305032e-2_f64 * t13765 - 0.22866142996303859718e-3_f64 * t13779 - 0.68026775414003982663e-1_f64 * t13781 + 0.25724410870841842183e-2_f64 * t3934 * t22829 + 0.12862205435420921092e-2_f64 * t5671 * t22833 + 0.25724410870841842183e-2_f64 * t3934 * t22837;
    t22840
}
