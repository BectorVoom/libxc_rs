//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2367/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2367(t15857: f64, t1873: f64, t652: f64, t1874: f64, t45632: f64, t12841: f64, t1774: f64, t1849: f64, t22461: f64, t22559: f64, t2320: f64, t23855: f64, t4037: f64, t510: f64, t6517: f64, t7670: f64, t90352: f64, t91752: f64, t91755: f64, t91757: f64, t91759: f64, t91762: f64, t91763: f64, t91765: f64, t91767: f64, t91769: f64, t91771: f64, t91777: f64) -> f64 {
    let t91780 = 2.0_f64 * t652 * t15857 * t1873;
    let t91782 = 2.0_f64 * t45632 * t1874;
    let t91789 = -2.0_f64 * t12841 * t6517 - t1774 * t22559 + t1849 * t23855 - 4.0_f64 * t22461 * t4037 - 2.0_f64 * t2320 * t7670 - 2.0_f64 * t510 * t90352 - t91752 - t91755 - t91757 - t91759 - t91762 - t91763 - t91765 - t91767 + t91769 - t91771 - t91777 - t91780 - t91782;
    t91789
}
