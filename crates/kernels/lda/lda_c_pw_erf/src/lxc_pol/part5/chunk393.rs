//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 393/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk393<F: Float>(t50: F, t34: F, t52: F, t1789: F, t352: F, t462: F, t1788: F, t59: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t1792 = t52 * t34;
    let t1796 = piecewise3::<f64>(t51, F::new(0.0), F::new(4.0) / F::new(9.0) * t1789 * t352 - F::new(8.0) / F::new(3.0) * t1792 * t462);
    let t1798 = (t1788 + t1796) * t59;
    t1798
}
