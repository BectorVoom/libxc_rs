//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1261/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1261<F: Float>(t44257: F, t9035: F, t2300: F, t2343: F, t2345: F, t3814: F, t44710: F, t45863: F, t45887: F, t49178: F, t49986: F, t50019: F, t50027: F, t50036: F, t50041: F, t50043: F, t904: F, t914: F, t916: F, t929: F) -> (F, F) {
    let t50045 = t9035 * t44257 / F::cast_from(4.0_f64);
    let t50046 = t49986 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t45863 - t914 * t916 * t904 * t50019 / F::cast_from(1536.0_f64) - t50027 + t2343 * t2345 * t44710 * t3814 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t929 * t2300 * t904 * t49178 + t50036 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t45887 + t50041 - t50043 + t50045;
    (t50045, t50046)
}
