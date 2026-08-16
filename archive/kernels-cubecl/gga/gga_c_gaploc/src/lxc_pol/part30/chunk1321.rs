//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1321/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1321<F: Float>(t33557: F, t7572: F, t7573: F, t10903: F, t22854: F, t33627: F, t10867: F, t28976: F, t10925: F, t5715: F, t32260: F, t739: F) -> (F, F, F, F, F, F) {
    let t33666 = F::cast_from(0.13803453343411469884e2_f64) * t7572 * t7573 * t33557;
    let t33668 = F::cast_from(0.13803453343411469884e2_f64) * t22854 * t10903;
    let t33671 = F::cast_from(0.13803453343411469884e2_f64) * t7572 * t7573 * t33627;
    let t33673 = F::cast_from(0.50050685932590597338e1_f64) * t10867 * t28976;
    let t33675 = F::cast_from(0.47667319935800568892e0_f64) * t10925 * t5715;
    let t33676 = t739 * t32260;
    (t33666, t33668, t33671, t33673, t33675, t33676)
}
