//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 797/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk797<F: Float>(t1764: F, t7824: F, t2816: F, t595: F, t637: F, t1734: F, t2758: F, t5777: F, t5793: F, t5812: F, t5815: F, t5920: F, t5923: F, t5925: F, t5927: F, t7825: F) -> (F,) {
    let t7827 = t7824 * t1764;
    let t7829 = t595 * t2816;
    let t7831 = 0.40020429009866666666e-2 * t7829 * t637;
    let t7832 = t2758 * t1734;
    let t7834 = -0.20010214504933333333e-2 * t5920 - t5777 - 0.32106488758451047386e0 * t7825 + 0.21687162600603479684e-1 * t7827 - t5793 - t7831 + 0.26680286006577777777e-2 * t7832 + t5923 + t5812 + t5815 + t5925 - t5927;
    (t7834,)
}
