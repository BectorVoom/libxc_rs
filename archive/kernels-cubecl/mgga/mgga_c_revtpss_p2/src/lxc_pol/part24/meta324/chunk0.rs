//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1125/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1125<F: Float>(t13765: F, t13779: F, t13781: F, t1410: F, t22023: F, t22028: F, t22030: F, t22815: F, t22822: F, t22829: F, t22833: F, t22837: F, t3934: F, t5671: F, t9711: F, t9725: F, t9729: F) -> F {
    let t22840 = -F::cast_from(0.25724410870841842183e-1_f64) * t1410 * t22815 + F::cast_from(0.21437009059034868486e-4_f64) * t22023 - F::cast_from(0.42874018118069736972e-4_f64) * t22028 + F::cast_from(0.12004725073059526352e-1_f64) * t22030 + t9711 + t9725 - t9729 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t22822 + F::cast_from(0.16262400898971305032e-2_f64) * t13765 - F::cast_from(0.22866142996303859718e-3_f64) * t13779 - F::cast_from(0.68026775414003982663e-1_f64) * t13781 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t22829 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t22833 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t22837;
    t22840
}
