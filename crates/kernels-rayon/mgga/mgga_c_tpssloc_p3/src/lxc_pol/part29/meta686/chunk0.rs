//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2350/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2350(t12823: f64, t15857: f64, t2114: f64, t2312: f64, t2314: f64, t2323: f64, t27290: f64, t27858: f64, t27863: f64, t27879: f64, t4034: f64, t5107: f64, t5361: f64, t574: f64, t652: f64, t671: f64, t672: f64, t7264: f64, t7412: f64, t7989: f64, t8103: f64, t91763: f64, t91765: f64, t91767: f64, t91769: f64, t91771: f64, t91777: f64, t91780: f64, t91782: f64, t96238: f64, t96269: f64, t96271: f64) -> f64 {
    let t96274 = -t91763 - t91765 - t91767 + t91769 - t91771 - t91777 - t91780 - t91782 - t2114 * t15857 - 2.0_f64 * t7264 * t5107 - 4.0_f64 * t27863 * t2323 - 4.0_f64 * t96238 * t672 - 4.0_f64 * t2314 * t27290 - 2.0_f64 * t12823 * t7989 - 4.0_f64 * t4034 * t27879 - 4.0_f64 * t652 * t27858 * t671 + 2.0_f64 * t7412 * t5361 - t2312 * t8103 + (t96269 + t96271) * t574;
    t96274
}
