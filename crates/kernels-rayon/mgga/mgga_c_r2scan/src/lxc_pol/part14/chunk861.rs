//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 861/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk861(t625: f64, t898: f64, t1768: f64, t1764: f64, t2816: f64, t595: f64, t637: f64, t1734: f64, t2758: f64, t5777: f64, t5793: f64, t5812: f64, t5815: f64, t5920: f64, t5923: f64, t5925: f64, t5927: f64) -> f64 {
    let t7824 = t898 * t625;
    let t7825 = t7824 * t1768;
    let t7827 = t7824 * t1764;
    let t7829 = t595 * t2816;
    let t7831 = 0.40020429009866666666e-2_f64 * t7829 * t637;
    let t7832 = t2758 * t1734;
    let t7834 = -0.20010214504933333333e-2_f64 * t5920 - t5777 - 0.32106488758451047386e0_f64 * t7825 + 0.21687162600603479684e-1_f64 * t7827 - t5793 - t7831 + 0.26680286006577777777e-2_f64 * t7832 + t5923 + t5812 + t5815 + t5925 - t5927;
    t7834
}
