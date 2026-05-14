//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1029/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1029<F: Float>(t14094: F, t689: F, t2435: F, t5600: F, t1426: F, t1893: F, t786: F, t3917: F, t136: F, t1903: F, t2457: F, t9674: F, t10175: F, t5722: F, t122: F, t5721: F) -> (F, F, F, F, F, F, F) {
    let t14096 = 0.10975748638225852664e-1 * t689 * t14094;
    let t14097 = t2435 * t5600;
    let t14099 = t1893 * t1426;
    let t14100 = t786 * t14099;
    let t14102 = 0.19514881078765566038e-1 * t14100 * t3917;
    let t14103 = t1903 * t136;
    let t14104 = t14103 * t2457;
    let t14105 = t9674 * t14104;
    let t14108 = 0.19514881078765566038e-1 * t10175 * t5722;
    let t14109 = t5721 * t122;
    (t14096, t14097, t14100, t14102, t14105, t14108, t14109)
}
