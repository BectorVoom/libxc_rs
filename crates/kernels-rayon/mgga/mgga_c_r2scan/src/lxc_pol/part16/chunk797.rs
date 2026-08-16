//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 797/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk797(t1764: f64, t7824: f64, t2816: f64, t595: f64, t637: f64, t1734: f64, t2758: f64, t5986: f64, t2461: f64, t759: f64, t761: f64, t2049: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7827 = t7824 * t1764;
    let t7829 = t595 * t2816;
    let t7831 = 0.40020429009866666666e-2_f64 * t7829 * t637;
    let t7832 = t2758 * t1734;
    let t7849 = 80.0_f64 * t5986;
    let t7861 = 0.571528e-1_f64 * t759 * t2461 * t761;
    let t7865 = t759 * t955 * t2049;
    (t7827, t7831, t7832, t7849, t7861, t7865)
}
