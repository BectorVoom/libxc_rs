//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 739/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk739<F: Float>(t5156: F, t1508: F, t1511: F, t1536: F, t1634: F, t1816: F, t2536: F, t2718: F, t4025: F, t4996: F, t5005: F, t5130: F, t5132: F, t5134: F, t5139: F, t5141: F, t5144: F, t5148: F, t5150: F, t5154: F) -> (F, F, F, F) {
    let t5157 = F::cast_from(0.17544670867903938621e1_f64) * t5156;
    let t5158 = t1511 * t1508;
    let t5159 = F::cast_from(0.51947577317044391276e2_f64) * t5158;
    let t5160 = F::new(18.0) * t1536 * t1634 * t2718 - F::new(3.0) * t1816 * t2536 * t4025 + t4996 + t5005 - t5130 + t5132 - t5134 - t5139 + t5141 - t5144 - t5148 + t5150 - t5154 - t5157 - t5159;
    (t5157, t5158, t5159, t5160)
}
