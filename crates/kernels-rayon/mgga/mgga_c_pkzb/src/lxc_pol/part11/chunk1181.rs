//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1181/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1181(t29017: f64, t600: f64, t10502: f64, t179: f64, t19910: f64, t20267: f64, t24038: f64, t2575: f64, t2592: f64, t28977: f64, t28979: f64, t28990: f64, t28992: f64, t28995: f64, t28999: f64, t29001: f64, t29004: f64, t29008: f64, t29010: f64, t29013: f64, t51: f64, t568: f64, t612: f64, t6896: f64, t6990: f64, t8821: f64) -> (f64, f64) {
    let t29018 = t29017 * t600;
    let t29022 = t19910 + 0.12004725073059526352e0_f64 * t28977 + 0.60023625365297631763e-2_f64 * t28979 + 0.18007087609589289528e0_f64 * t612 * t20267 * t51 * t10502 * t568 - 0.77173232612525526549e-1_f64 * t612 * t6990 * t8821 * t2575 - 0.60023625365297631763e-2_f64 * t28990 + 0.10003937560882938627e-2_f64 * t28992 + 7.0_f64 / 48.0_f64 * t24038 + 0.38586616306262763276e-2_f64 * t2592 * t179 * t28995 - 0.12004725073059526352e-1_f64 * t28999 - 0.12004725073059526352e-1_f64 * t29001 + 0.30011812682648815881e-2_f64 * t2592 * t179 * t29004 - 0.60023625365297631763e-2_f64 * t29008 + 0.3001181268264881588e-2_f64 * t29010 - 0.77173232612525526552e-2_f64 * t6896 * t179 * t29013 + 0.42874018118069736972e-3_f64 * t2592 * t179 * t29018;
    (t29018, t29022)
}
