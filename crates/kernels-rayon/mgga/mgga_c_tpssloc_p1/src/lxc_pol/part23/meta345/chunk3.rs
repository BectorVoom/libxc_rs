//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1136/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1136(t39706: f64, t39749: f64, t39803: f64, t39840: f64, t17: f64, t521: f64, t1287: f64, t9216: f64, t11985: f64, t25: f64, t514: f64, t11998: f64, t28: f64, t517: f64) -> (f64, f64, f64, f64, f64) {
    let t39842 = t39706 + t39749 + t39803 + t39840;
    let t39844 = t17 * t521 * t39842;
    let t39855 = t9216 * t1287;
    let t39856 = 960.0_f64 * t39855;
    let t39861 = 1.0_f64 / t514 / t11985 / t25;
    let t39877 = 1.0_f64 / t517 / t11998 / t28;
    (t39842, t39844, t39856, t39861, t39877)
}
