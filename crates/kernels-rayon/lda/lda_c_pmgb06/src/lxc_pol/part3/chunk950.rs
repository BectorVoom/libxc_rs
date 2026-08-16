//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 950/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk950(t8781: f64, t8785: f64, t1105: f64, t2160: f64, t8738: f64, t8743: f64, t8746: f64, t8749: f64, t8751: f64, t8755: f64, t8759: f64, t8760: f64, t8762: f64, t8769: f64, t8774: f64, t8779: f64, t8787: f64, t8794: f64) -> f64 {
    let t11132 = 960.0_f64 * t8781;
    let t11133 = 192.0_f64 * t8785;
    let t11135 = t1105 * t2160;
    let t11136 = 36.0_f64 * t11135;
    let t11137 = -10.526802520742363_f64 * t8738 - t8743 + t8746 - 24.0_f64 * t8749 - 4.0_f64 * t8751 - t8755 - t8759 + 10.526802520742363_f64 * t8760 - 155.84273195113317_f64 * t8762 + t8769 - t8774 + t8779 - t11132 + t11133 - 0.0017090684152272775_f64 * t8787 - t8794 + t11136;
    t11137
}
