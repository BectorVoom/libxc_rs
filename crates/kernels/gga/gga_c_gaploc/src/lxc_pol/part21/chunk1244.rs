//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1244/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1244<F: Float>(t33047: F, t10938: F, t1980: F, t2028: F, t32757: F, t32970: F, t326: F, t32948: F, t825: F, t11109: F, t5840: F, t10856: F, t2033: F, t549: F) -> (F, F, F, F, F, F) {
    let t33048 = F::cast_from(0.29792074959875355558e-1_f64) * t33047;
    let t33055 = F::cast_from(0.79445533226334281486e-1_f64) * t1980 * t10938 * t2028;
    let t33060 = F::cast_from(0.50050685932590597338e1_f64) * t32757 * t32970;
    let t33067 = F::cast_from(0.18404604457881959845e2_f64) * t825 * t326 * t32948;
    let t33068 = t5840 * t11109;
    let t33069 = F::cast_from(0.51123901271894332902e0_f64) * t33068;
    let t33071 = t2033 * t549 * t10856;
    (t33048, t33055, t33060, t33067, t33069, t33071)
}
