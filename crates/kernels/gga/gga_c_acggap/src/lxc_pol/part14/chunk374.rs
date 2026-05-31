//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 374/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk374<F: Float>(t1795: F, t336: F, t368: F, t1501: F, t495: F, t1734: F, t337: F, t513: F, t535: F, t1150: F, t1180: F, t127: F, t1353: F, t1355: F, t1424: F, t1435: F, t1761: F, t1775: F, t1784: F, t1788: F, t335: F, t367: F) -> (F, F, F, F, F) {
    let t1797 = t336 * t368 * t1795;
    let t1801 = t336 * t1501 * t495;
    let t1805 = t336 * t337 * t1734;
    let t1809 = t336 * t535 * t513;
    let t1812 = F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1761 + t127 * t1775 / F::cast_from(96.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t1353 - F::cast_from(0.42874018118069736972e-3_f64) * t1355 + F::cast_from(0.17149607247227894789e-2_f64) * t1424 + F::cast_from(0.85748036236139473944e-3_f64) * t1435 + t367 * t1784 / F::cast_from(48.0_f64) + t1150 * t1788 / F::cast_from(16.0_f64) - t367 * t1797 / F::cast_from(96.0_f64) - t335 * t1801 / F::cast_from(24.0_f64) - t335 * t1805 / F::cast_from(48.0_f64) - t367 * t1809 / F::cast_from(48.0_f64);
    (t1797, t1801, t1805, t1809, t1812)
}
