//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1136/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1136<F: Float>(t4260: F, t883: F, t6490: F, t6525: F, t1436: F, t9544: F, t1538: F, t20395: F, t6583: F, t20481: F, t21414: F, t123: F, t6393: F) -> (F, F, F, F, F, F) {
    let t30204 = t4260 * t883;
    let t30207 = F::new(0.94850022118920498664e-2) * t6525 * t30204 * t6490;
    let t30246 = t1436 * t9544;
    let t30250 = t6583 * t1538 * t883 * t20395;
    let t30253 = F::new(0.59584149919750711116e-1) * t20481 * t21414;
    let t30258 = t6393 * t123 * t883;
    (t30204, t30207, t30246, t30250, t30253, t30258)
}
