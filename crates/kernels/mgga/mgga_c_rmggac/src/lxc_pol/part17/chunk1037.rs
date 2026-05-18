//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1037/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1037<F: Float>(t1356: F, t35707: F, t35713: F, t35717: F, t35720: F, t35724: F, t35729: F, t35742: F, t35744: F, t4041: F, t40480: F, t40506: F, t40596: F, t47030: F, t47032: F, t47037: F, t47042: F, t5267: F, t5888: F, t8800: F, t884: F, t9944: F) -> F {
    let t47044 = -F::new(0.23948483403727617128e0) * t884 * t8800 * t5267 - F::new(0.23948483403727617128e0) * t1356 * t40596 * t5888 + F::new(0.30487649791575028314e-3) * t35707 + t35713 + t35717 - F::new(0.43368970657079495312e-4) * t35720 - F::new(0.43368970657079495312e-4) * t35724 - t35729 + F::new(0.15243824895787514157e-3) * t35742 + F::new(0.15243824895787514157e-3) * t35744 + F::new(0.11974241701863808564e0) * t4041 * t9944 + t40480 - F::new(0.53205749866622299248e-5) * t47030 + F::new(0.24829349937757072983e-4) * t47032 - t40506 - F::new(0.12769379967989351819e-3) * t47037 - F::new(0.31923449919973379548e-4) * t47042;
    t47044
}
