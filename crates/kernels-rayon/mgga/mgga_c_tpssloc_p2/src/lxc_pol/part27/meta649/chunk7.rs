//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2257/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2257(t25927: f64, t86781: f64, t1877: f64, t1915: f64, t22959: f64, t23286: f64, t23290: f64, t25013: f64, t2522: f64, t25928: f64, t25945: f64, t28: f64, t6670: f64, t7649: f64, t86703: f64, t86734: f64, t86751: f64, t86757: f64, t87945: f64, t89881: f64, t89888: f64, t89892: f64, t89896: f64, t89904: f64, t89907: f64, t89911: f64) -> f64 {
    let t89917 = t25927 * t86781;
    let t89920 = 3.0_f64 / 2.0_f64 * t2522 * t1915 * t89881 + 3.0_f64 / 2.0_f64 * t2522 * t23286 * t7649 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t89888 + 3.0_f64 * t2522 * t1915 * t89892 + 6.0_f64 * t25013 * t89896 + t1877 * t87945 * t28 / 2.0_f64 + 2.0_f64 * t86703 * t25928 + t86734 + 3.0_f64 * t22959 * t89904 + t86751 - t1877 * t6670 * t89907 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t89911 - t86757 - t1877 * t23290 * t25945 + 6.0_f64 * t22959 * t89917;
    t89920
}
