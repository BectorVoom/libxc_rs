//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1252/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1252<F: Float>(t1092: F, t1121: F, t6689: F, t95655: F, t1133: F, t27763: F, t70032: F, t1020: F, t19706: F, t7718: F, t100436: F, t26955: F, t26966: F, t27070: F, t28184: F, t28204: F, t29112: F, t29161: F, t8087: F, t96977: F, t96980: F, t96993: F, t97010: F) -> (F, F, F, F) {
    let t100447 = t1092 * t95655 * t6689 * t1121;
    let t100451 = t1092 * t27763 * t70032 * t1133;
    let t100456 = t1020 * t7718 * t19706;
    let t100458 = -F::cast_from(0.30918233506944444445e-4_f64) * t26955 * t100436 + t96977 - t96980 - F::cast_from(0.92754700520833333334e-4_f64) * t27070 * t29161 - F::cast_from(0.92754700520833333334e-4_f64) * t28204 * t28184 + t96993 - F::cast_from(0.18534722222222222222e-2_f64) * t97010 * t8087 - F::cast_from(0.10446625e-1_f64) * t100447 + F::cast_from(0.23214722222222222221e-2_f64) * t100451 + F::cast_from(0.41188271604938271605e-3_f64) * t26966 * t29112 + F::cast_from(0.77382407407407407407e-3_f64) * t100456;
    (t100447, t100451, t100456, t100458)
}
