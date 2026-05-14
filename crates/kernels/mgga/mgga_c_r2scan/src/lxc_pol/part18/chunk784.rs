//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 784/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk784<F: Float>(t5871: F, t5878: F, t7026: F, t7027: F, t170: F, t3129: F, t584: F, t591: F, t159: F, t5774: F, t5777: F, t5793: F, t5919: F, t5920: F, t5923: F, t7825: F, t7827: F, t7831: F, t7832: F) -> (F, F) {
    let t9005 = -t5871 - t7026 + t7027 + t5878;
    let t9006 = t9005 * t170;
    let t9010 = t584 * t3129 * t591;
    let t9012 = -t5774 + t5919 - 0.10005107252466666667e-2 * t5920 - t5777 - 0.64212977516902094771e0 * t7825 + 0.43374325201206959368e-1 * t7827 - t5793 - t7831 + 0.53360572013155555555e-2 * t7832 + 0.285764e-1 * t159 * t9006 + t5923 - 0.571528e-1 * t9010;
    (t9005, t9012)
}
