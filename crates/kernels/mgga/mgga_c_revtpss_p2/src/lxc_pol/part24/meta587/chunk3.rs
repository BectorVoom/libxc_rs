//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1827/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1827<F: Float>(t4003: F, t92069: F, t1390: F, t1410: F, t1872: F, t22809: F, t3944: F, t4002: F, t46885: F, t48829: F, t48833: F, t48849: F, t48853: F, t48879: F, t48909: F, t6816: F, t6836: F, t6849: F, t74485: F, t74491: F, t74493: F, t74511: F, t74522: F, t800: F, t828: F, t86112: F, t86124: F, t9748: F, t9942: F) -> (F, F) {
    let t92158 = t92069 * t4003;
    let t92168 = F::cast_from(0.45178982497454656791e-6_f64) * t48829 - F::new(3.0) / F::new(2.0) * t9748 * t800 * t6849 * t6816 + t3944 * t800 * t1872 * t22809 / F::new(4.0) + F::cast_from(0.11560105625909173524e-1_f64) * t48833 - F::cast_from(0.20553867802866510527e-1_f64) * t48849 + F::cast_from(0.28900264064772933811e-2_f64) * t48853 - F::cast_from(0.24009450146119052704e-1_f64) * t86112 + F::cast_from(0.2168591159877823526e-3_f64) * t74485 + F::cast_from(0.6098400337114239387e-3_f64) * t86124 - F::cast_from(0.18292914397043087775e-2_f64) * t74491 + F::cast_from(0.91464571985215438873e-2_f64) * t74493 + F::cast_from(0.32528867398167352889e-3_f64) * t48879 + F::cast_from(0.32524801797942610064e-2_f64) * t74511 + t46885 + F::cast_from(0.15246000842785598467e-4_f64) * t74522 - F::cast_from(0.32131292352189751911e-5_f64) * t48909 + F::cast_from(0.12862205435420921092e-2_f64) * t4002 * t1390 * t828 * t92158 - F::cast_from(0.1543464652250510531e0_f64) * t1410 * t9942 * t828 * t6836 * t6816;
    (t92158, t92168)
}
