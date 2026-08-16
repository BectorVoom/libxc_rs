//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1364/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1364(t31682: f64, t3966: f64, t8308: f64, t1409: f64, t8513: f64, t8514: f64, t1433: f64, t1862: f64, t113875: f64, t645: f64, t12571: f64, t31680: f64) -> (f64, f64, f64, f64) {
    let t121044 = t8308 * t31682 * t3966;
    let t121050 = t8513 * t8514 * t1409;
    let t121053 = t1862 * t1433;
    let t121055 = t113875 * t121053 * t645;
    let t121058 = t12571 * t31680;
    (t121044, t121050, t121055, t121058)
}
