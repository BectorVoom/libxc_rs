//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3160/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3160<F: Float>(t12486: F, t1756: F, t12485: F, t1749: F, t12423: F, t12476: F, t12481: F, t12487: F, t12488: F, t12501: F, t12514: F, t12553: F, t16971: F, t16974: F, t17032: F, t1757: F, t3497: F, t3521: F, t45163: F, t5181: F, t5184: F, t57831: F, t57833: F, t57835: F, t57837: F, t57840: F, t57856: F) -> F {
    let t58259 = t12486 * t1756;
    let t58262 = t1749 * t12485;
    let t58275 = F::cast_from(0.5848223622634646207e0_f64) * t45163 * t1757 + F::cast_from(0.17544670867903938621e1_f64) * t12476 * t5181 + F::new(6.0) * t17032 * t12514 + t57831 + t57833 - t57835 - t57837 + t57840 - F::cast_from(0.31168546390226634766e3_f64) * t58259 * t12501 - F::cast_from(0.10389515463408878255e3_f64) * t58262 * t12488 - t57856 + F::cast_from(0.10526802520742363173e2_f64) * t12481 * t16971 + F::cast_from(0.10526802520742363173e2_f64) * t3521 * t5181 * t3497 + F::cast_from(0.6233709278045326953e3_f64) * t12553 * t5184 * t12487 + F::new(18.0) * t12423 * t16974;
    t58275
}
