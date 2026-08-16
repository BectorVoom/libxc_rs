//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2350/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2350<F: Float>(t12823: F, t15857: F, t2114: F, t2312: F, t2314: F, t2323: F, t27290: F, t27858: F, t27863: F, t27879: F, t4034: F, t5107: F, t5361: F, t574: F, t652: F, t671: F, t672: F, t7264: F, t7412: F, t7989: F, t8103: F, t91763: F, t91765: F, t91767: F, t91769: F, t91771: F, t91777: F, t91780: F, t91782: F, t96238: F, t96269: F, t96271: F) -> F {
    let t96274 = -t91763 - t91765 - t91767 + t91769 - t91771 - t91777 - t91780 - t91782 - t2114 * t15857 - F::cast_from(2.0_f64) * t7264 * t5107 - F::cast_from(4.0_f64) * t27863 * t2323 - F::cast_from(4.0_f64) * t96238 * t672 - F::cast_from(4.0_f64) * t2314 * t27290 - F::cast_from(2.0_f64) * t12823 * t7989 - F::cast_from(4.0_f64) * t4034 * t27879 - F::cast_from(4.0_f64) * t652 * t27858 * t671 + F::cast_from(2.0_f64) * t7412 * t5361 - t2312 * t8103 + (t96269 + t96271) * t574;
    t96274
}
