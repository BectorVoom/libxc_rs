//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 570/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk570<F: Float>(t7755: F, t7756: F, t2010: F, t290: F, t7556: F) -> (F, F, F) {
    let t7757 = t7755 * t7756;
    let t7758 = t2010 * t7757;
    let t7759 = F::new(0.72042316457491791906e-3) * t7758;
    let t7760 = t290 * t7556;
    (t7757, t7759, t7760)
}
