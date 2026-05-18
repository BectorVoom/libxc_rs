//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 502/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk502<F: Float>(t12: F, t24: F, t1642: F, t972: F, t8: F, t87: F, t1429: F, t439: F, t1003: F, t1651: F, t91: F, t507: F, t98: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t2540 = t1642 * t972;
    let t2543 = t87 * t8;
    let t2547 = piecewise3::<f64>(t84, F::new(0.0), F::new(4.0) / F::new(9.0) * t2540 * t439 + F::new(8.0) / F::new(3.0) * t2543 * t1429);
    let t2548 = t1651 * t1003;
    let t2551 = t91 * t8;
    let t2555 = piecewise3::<f64>(t90, F::new(0.0), F::new(4.0) / F::new(9.0) * t2548 * t507 - F::new(8.0) / F::new(3.0) * t2551 * t1429);
    let t2557 = (t2547 + t2555) * t98;
    (t2540, t2548, t2557)
}
