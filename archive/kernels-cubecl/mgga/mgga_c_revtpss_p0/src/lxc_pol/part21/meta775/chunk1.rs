//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2759/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2759<F: Float>(t10703: F, t2674: F, t50789: F, t10666: F, t2745: F, t2747: F, t2749: F, t40737: F, t40744: F, t40748: F, t40750: F, t40753: F, t40759: F, t40761: F, t40765: F, t40771: F, t4364: F, t4365: F, t50459: F, t50752: F, t50754: F, t50757: F, t50758: F, t50771: F, t50774: F, t50784: F) -> F {
    let t50791 = t2674 * t10703 * t50789;
    let t50793 = F::cast_from(0.60023625365297631762e-1_f64) * t50752 - F::cast_from(0.12004725073059526352e-1_f64) * t50754 + F::cast_from(0.51448821741683684368e-2_f64) * t50757 * t4364 * t4365 * t50758 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t4365 * t10666 - F::cast_from(0.17149607247227894789e-3_f64) * t50771 + t50774 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t50459 * t2749 + t40737 + F::cast_from(0.15246000842785598467e-3_f64) * t40744 - F::cast_from(0.12004725073059526352e-1_f64) * t40748 + F::cast_from(0.30011812682648815881e-2_f64) * t40750 - F::cast_from(0.38115002106963996168e-4_f64) * t50784 - F::cast_from(0.13553694749236397037e-4_f64) * t40753 - t40759 - F::cast_from(0.13553694749236397037e-4_f64) * t40761 + F::cast_from(0.24396650548625514668e-3_f64) * t40765 + t40771 + F::cast_from(0.15246000842785598468e-2_f64) * t50791;
    t50793
}
