//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1073/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1073<F: Float>(t10574: F, t6966: F, t10627: F, t5391: F, t600: F, t10621: F, t1721: F, t10502: F, t179: F, t19910: F, t20267: F, t24038: F, t2575: F, t2592: F, t28977: F, t28979: F, t28990: F, t28992: F, t28995: F, t28999: F, t29001: F, t29004: F, t29008: F, t51: F, t568: F, t612: F, t6896: F, t6990: F, t8821: F) -> (F, F, F, F) {
    let t29010 = t6966 * t10574;
    let t29012 = t10627 * t5391;
    let t29013 = t29012 * t600;
    let t29017 = t10621 * t1721;
    let t29018 = t29017 * t600;
    let t29022 = t19910 + 0.12004725073059526352e0 * t28977 + 0.60023625365297631763e-2 * t28979 + 0.18007087609589289528e0 * t612 * t20267 * t51 * t10502 * t568 - 0.77173232612525526549e-1 * t612 * t6990 * t8821 * t2575 - 0.60023625365297631763e-2 * t28990 + 0.10003937560882938627e-2 * t28992 + 7.0 / 48.0 * t24038 + 0.38586616306262763276e-2 * t2592 * t179 * t28995 - 0.12004725073059526352e-1 * t28999 - 0.12004725073059526352e-1 * t29001 + 0.30011812682648815881e-2 * t2592 * t179 * t29004 - 0.60023625365297631763e-2 * t29008 + 0.3001181268264881588e-2 * t29010 - 0.77173232612525526552e-2 * t6896 * t179 * t29013 + 0.42874018118069736972e-3 * t2592 * t179 * t29018;
    (t29012, t29013, t29018, t29022)
}
