//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1102/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1102<F: Float>(t343: F, t50018: F, t13403: F, t2170: F, t3138: F, t44254: F, t45882: F, t2210: F, t49178: F, t858: F, t884: F, t11630: F, t11773: F, t44257: F, t9035: F, t2300: F, t2343: F, t2345: F, t3814: F, t44710: F, t45863: F, t45887: F, t49986: F, t904: F, t914: F, t916: F, t929: F) -> (F, F, F, F, F, F, F) {
    let t50019 = t50018 * t343;
    let t50027 = t3138 * t2170 * t44254 * t13403 / 2.0;
    let t50036 = 7.0 / 12.0 * t45882;
    let t50041 = t884 * t2210 * t858 * t49178 / 4.0;
    let t50043 = t11773 * t11630 / 16.0;
    let t50045 = t9035 * t44257 / 4.0;
    let t50046 = t49986 - 7.0 / 288.0 * t45863 - t914 * t916 * t904 * t50019 / 1536.0 - t50027 + t2343 * t2345 * t44710 * t3814 / 96.0 + 5.0 / 192.0 * t929 * t2300 * t904 * t49178 + t50036 - 7.0 / 48.0 * t45887 + t50041 - t50043 + t50045;
    (t50019, t50027, t50036, t50041, t50043, t50045, t50046)
}
