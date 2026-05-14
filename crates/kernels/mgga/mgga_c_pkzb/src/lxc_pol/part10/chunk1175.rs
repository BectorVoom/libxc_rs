//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1175/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1175<F: Float>(t2280: F, t2368: F, t5728: F, t2387: F, t6517: F, t486: F, t931: F, t154: F, t2226: F, t385: F, t1478: F, t405: F, t824: F, t2185: F, t6446: F, t178: F, t404: F, t4902: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18888 = t2280 * t2280;
    let t18889 = 1.0 / t18888;
    let t18979 = t2368 * t5728;
    let t18980 = t6517 * t2387;
    let t18989 = t486 * t931;
    let t18992 = t385 * t154 * t18989 * t2226;
    let t19023 = t1478 * t405;
    let t19026 = t385 * t154 * t19023 * t824;
    let t19030 = t385 * t154 * t6446 * t2185;
    let t19055 = 0.14820648238345094262e-3 * t404 * t178 * t4902 * t405;
    (t18889, t18979, t18980, t18989, t18992, t19023, t19026, t19030, t19055)
}
