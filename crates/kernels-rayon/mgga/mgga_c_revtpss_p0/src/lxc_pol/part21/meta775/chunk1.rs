//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2759/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2759(t10703: f64, t2674: f64, t50789: f64, t10666: f64, t2745: f64, t2747: f64, t2749: f64, t40737: f64, t40744: f64, t40748: f64, t40750: f64, t40753: f64, t40759: f64, t40761: f64, t40765: f64, t40771: f64, t4364: f64, t4365: f64, t50459: f64, t50752: f64, t50754: f64, t50757: f64, t50758: f64, t50771: f64, t50774: f64, t50784: f64) -> f64 {
    let t50791 = t2674 * t10703 * t50789;
    let t50793 = 0.60023625365297631762e-1_f64 * t50752 - 0.12004725073059526352e-1_f64 * t50754 + 0.51448821741683684368e-2_f64 * t50757 * t4364 * t4365 * t50758 - 0.21437009059034868486e-3_f64 * t2745 * t4364 * t4365 * t10666 - 0.17149607247227894789e-3_f64 * t50771 + t50774 + 0.25724410870841842183e-2_f64 * t2745 * t2747 * t50459 * t2749 + t40737 + 0.15246000842785598467e-3_f64 * t40744 - 0.12004725073059526352e-1_f64 * t40748 + 0.30011812682648815881e-2_f64 * t40750 - 0.38115002106963996168e-4_f64 * t50784 - 0.13553694749236397037e-4_f64 * t40753 - t40759 - 0.13553694749236397037e-4_f64 * t40761 + 0.24396650548625514668e-3_f64 * t40765 + t40771 + 0.15246000842785598468e-2_f64 * t50791;
    t50793
}
