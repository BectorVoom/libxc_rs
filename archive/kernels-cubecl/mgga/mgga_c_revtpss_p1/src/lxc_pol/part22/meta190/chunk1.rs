//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1212/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1212<F: Float>(t1559: F, t213: F, t234: F, t2776: F, t2780: F, t2787: F, t2791: F, t2796: F, t2802: F, t2806: F, t2810: F, t2815: F, t4366: F, t4424: F, t4469: F, t4494: F, t4497: F, t4501: F, t4504: F, t4514: F, t4520: F, t4524: F, t4526: F, t820: F, t837: F, t879: F) -> F {
    let t4533 = t2776 - t2780 + F::cast_from(0.54878743191129263322e-2_f64) * t2787 - F::cast_from(0.54878743191129263322e-2_f64) * t2791 + t2796 - F::cast_from(0.9757440539382783019e-2_f64) * t2802 + F::cast_from(0.9757440539382783019e-2_f64) * t2806 - t2810 + F::cast_from(0.54878743191129263322e-2_f64) * t4497 - F::cast_from(0.9757440539382783019e-2_f64) * t4501 + F::cast_from(0.13170898365871023197e1_f64) * t4504 * t4494 * t4366 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t2815 * t1559 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t4424 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t4494 * t837 - F::cast_from(0.54878743191129263322e-2_f64) * t4520 + F::cast_from(0.9757440539382783019e-2_f64) * t4524 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4526 * t837 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t234 * t4469;
    t4533
}
