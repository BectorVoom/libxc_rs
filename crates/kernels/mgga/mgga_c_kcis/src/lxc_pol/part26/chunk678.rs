//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 678/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk678<F: Float>(t7609: F, t826: F, t2153: F, t2533: F, t2538: F, t113: F, t805: F, t774: F, t808: F, t153: F, t740: F, t2150: F, t815: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7610 = t7609 * t826;
    let t7611 = t2533 * t2153;
    let t7612 = t2153 * t826;
    let t7613 = t2538 * t7612;
    let t7614 = F::new(2.0) * t7613;
    let t7615 = t805 * t113;
    let t7617 = t113 * t774;
    let t7618 = t808 * t7617;
    let t7620 = t153 * t740;
    let t7622 = t815 * t2150;
    (t7610, t7611, t7612, t7614, t7615, t7617, t7618, t7620, t7622)
}
