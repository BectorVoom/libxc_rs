//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 996/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk996<F: Float>(t20160: F, t6206: F, t1309: F, t13805: F, t13807: F, t13810: F, t13824: F, t13827: F, t20142: F, t20151: F, t20155: F, t3966: F, t3970: F, t6189: F, t6207: F, t2160: F, t3981: F) -> (F, F) {
    let t20161 = t20160 * t6206;
    let t20162 = t1309 * t20161;
    let t20164 = 0.2398771828823642295e-1 * t13805 - 0.35981577432354634426e-1 * t13807 - 0.17990788716177317213e-1 * t13810 + 0.95950873152945691802e-1 * t13824 + 0.35981577432354634426e-1 * t13827 + 0.21588946459412780656e0 * t1309 * t20142 + 0.71963154864709268852e-1 * t3966 * t6189 - 0.1919017463058913836e0 * t3970 * t6189 - 0.83957014008827480328e-1 * t20151 - 0.32383419689119170984e0 * t1309 * t20155 - 0.57570523891767415083e0 * t3970 * t6207 + 0.71963154864709268853e-1 * t20162;
    let t20169 = t2160 * t3981;
    (t20164, t20169)
}
