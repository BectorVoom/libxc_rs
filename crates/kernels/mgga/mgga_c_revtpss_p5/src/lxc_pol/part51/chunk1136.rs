//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1136/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1136<F: Float>(t119781: F, t119783: F, t126092: F, t247: F, t126046: F, t837: F, t119764: F, t119778: F, t119789: F, t126043: F, t126049: F, t126052: F, t126055: F, t126062: F, t126065: F, t126068: F, t126072: F, t126076: F, t126081: F, t126083: F, t126085: F, t126087: F, t126089: F) -> F {
    let t126095 = t119781 * t247 * t126092 * t119783;
    let t126099 = t119781 * t247 * t126046 * t837;
    let t126101 = -F::cast_from(0.17354086964223805049e-2_f64) * t126043 - F::cast_from(0.17354086964223805049e-2_f64) * t119764 + F::cast_from(0.112937867033921868e-2_f64) * t126049 - F::cast_from(0.14874931683620404328e-2_f64) * t126052 - F::cast_from(0.11156198762715303246e-2_f64) * t126055 - F::cast_from(0.3718732920905101082e-4_f64) * t119778 - F::cast_from(0.11156198762715303246e-2_f64) * t126062 - F::cast_from(0.14874931683620404328e-2_f64) * t126065 + F::cast_from(0.3718732920905101082e-3_f64) * t126068 - F::cast_from(0.7437465841810202164e-3_f64) * t126072 - F::cast_from(0.7437465841810202164e-3_f64) * t126076 - F::cast_from(0.14874931683620404328e-2_f64) * t126081 + F::cast_from(0.3718732920905101082e-3_f64) * t126083 + F::cast_from(0.3718732920905101082e-3_f64) * t126085 - F::cast_from(0.3718732920905101082e-4_f64) * t126087 + F::cast_from(0.66119071333692697238e-4_f64) * t126089 + F::cast_from(0.66119071333692697238e-4_f64) * t119789 + F::cast_from(0.28234466758480466999e-3_f64) * t126095 - F::cast_from(0.28234466758480466999e-3_f64) * t126099;
    t126101
}
