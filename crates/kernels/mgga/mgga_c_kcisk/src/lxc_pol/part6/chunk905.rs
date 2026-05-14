//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 905/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk905<F: Float>(t30534: F, t30769: F, t1328: F, t2173: F, t8063: F, t13440: F, t1375: F, t30294: F, t30273: F, t457: F, t158: F, t173: F, t25425: F, t25427: F, t25429: F, t25485: F, t25487: F, t25489: F, t25491: F, t25493: F) -> (F, F, F) {
    let t30770 = t30534 + t30769;
    let t30771 = t30770 * t1328;
    let t30774 = t8063 * t2173;
    let t30775 = t30774 * t13440;
    let t30787 = t1375 * t30294;
    let t30790 = t1375 * t30273;
    let t30793 = t457 * t30294;
    let t30801 = -0.4684e-2 * t25425 - 0.39624999999999999999e-2 * t25427 + 0.26416666666666666666e-2 * t25429 - 0.2016525e-4 * t173 * t30787 + 0.21078e-1 * t158 * t30790 + 0.3513e-2 * t158 * t30793 + 0.70578375e-4 * t25485 + 0.14052e-1 * t25487 - 0.352891875e-4 * t25489 + 0.4705225e-4 * t25491 - 0.28104e-1 * t25493;
    (t30771, t30775, t30801)
}
