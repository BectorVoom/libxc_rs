//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1827/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1827(t4003: f64, t92069: f64, t1390: f64, t1410: f64, t1872: f64, t22809: f64, t3944: f64, t4002: f64, t46885: f64, t48829: f64, t48833: f64, t48849: f64, t48853: f64, t48879: f64, t48909: f64, t6816: f64, t6836: f64, t6849: f64, t74485: f64, t74491: f64, t74493: f64, t74511: f64, t74522: f64, t800: f64, t828: f64, t86112: f64, t86124: f64, t9748: f64, t9942: f64) -> (f64, f64) {
    let t92158 = t92069 * t4003;
    let t92168 = 0.45178982497454656791e-6_f64 * t48829 - 3.0_f64 / 2.0_f64 * t9748 * t800 * t6849 * t6816 + t3944 * t800 * t1872 * t22809 / 4.0_f64 + 0.11560105625909173524e-1_f64 * t48833 - 0.20553867802866510527e-1_f64 * t48849 + 0.28900264064772933811e-2_f64 * t48853 - 0.24009450146119052704e-1_f64 * t86112 + 0.2168591159877823526e-3_f64 * t74485 + 0.6098400337114239387e-3_f64 * t86124 - 0.18292914397043087775e-2_f64 * t74491 + 0.91464571985215438873e-2_f64 * t74493 + 0.32528867398167352889e-3_f64 * t48879 + 0.32524801797942610064e-2_f64 * t74511 + t46885 + 0.15246000842785598467e-4_f64 * t74522 - 0.32131292352189751911e-5_f64 * t48909 + 0.12862205435420921092e-2_f64 * t4002 * t1390 * t828 * t92158 - 0.1543464652250510531e0_f64 * t1410 * t9942 * t828 * t6836 * t6816;
    (t92158, t92168)
}
