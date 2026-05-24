//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 935/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk935<F: Float>(t1935: F, t29575: F, t24473: F, t2580: F, t2586: F, t9085: F, t741: F, t29542: F, t29545: F, t29548: F, t29551: F, t29554: F, t29556: F, t29558: F, t29562: F, t29565: F, t29567: F, t29569: F, t29573: F) -> (F, F, F, F) {
    let t29576 = t1935 * t29575;
    let t29578 = t24473 * t2580;
    let t29580 = t2586 * t9085;
    let t29581 = t741 * t29580;
    let t29583 = -t29542 / F::new(24.0) + F::new(19.0) / F::new(48.0) * t29545 + t29548 / F::new(64.0) + F::new(3.0) / F::new(8.0) * t29551 - t29554 / F::new(192.0) + F::new(3.0) / F::new(8.0) * t29556 - F::new(3.0) / F::new(16.0) * t29558 - t29562 / F::new(192.0) - F::new(19.0) / F::new(36.0) * t29565 - t29567 / F::new(64.0) + t29569 / F::new(8.0) + t29573 / F::new(24.0) + t29576 / F::new(2.0) + F::new(3.0) / F::new(256.0) * t29578 + t29581 / F::new(12.0);
    (t29576, t29578, t29581, t29583)
}
