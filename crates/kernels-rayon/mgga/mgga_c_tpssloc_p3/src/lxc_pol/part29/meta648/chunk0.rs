//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2154/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2154(t214: f64, t4265: f64, t1880: f64, t6572: f64, t25055: f64, t81591: f64, t13049: f64, t13065: f64, t13072: f64, t13461: f64, t1492: f64, t22975: f64, t23150: f64, t25168: f64, t25170: f64, t259: f64, t4268: f64, t6627: f64, t6663: f64, t82154: f64, t82172: f64, t82174: f64, t82182: f64, t866: f64, t87746: f64, t87748: f64, t87754: f64, t87755: f64, t87758: f64, t87765: f64, t87773: f64, t87777: f64, t87779: f64) -> (f64, f64) {
    let t87782 = t214 * t4265;
    let t87784 = t1880 * t87782 * t6572;
    let t87786 = t81591 * t25055;
    let t87787 = 0.76763589786250567036e-1_f64 * t87786;
    let t87792 = -0.82246703342411321825e-2_f64 * t87746 - t82154 + 24.0_f64 * t25168 * t87748 * t13049 - t87754 - 12.0_f64 * t87755 * t25170 - 2.0_f64 * t87758 * t866 + 2.0_f64 * t4268 * t22975 - 0.19739208802178717238e0_f64 * t87765 + t1492 * t23150 * t259 + 0.82246703342411321824e-2_f64 * t82172 - t6627 * t13461 + 0.76763589786250567036e-1_f64 * t82174 - 0.82246703342411321825e-2_f64 * t87773 + t87777 + 0.82246703342411321824e-2_f64 * t87779 - 0.82246703342411321824e-2_f64 * t82182 - 0.16449340668482264365e-1_f64 * t87784 - t87787 + 4.0_f64 * t6627 * t13072 - 2.0_f64 * t13065 * t6663;
    (t87782, t87792)
}
