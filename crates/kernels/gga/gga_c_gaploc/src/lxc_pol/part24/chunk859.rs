//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 859/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk859<F: Float>(t9823: F, t9824: F, t165: F, t2530: F, t161: F, t2685: F, t2684: F, t2465: F, t2581: F, t2464: F, t3311: F, t7416: F, t2013: F, t3296: F, t969: F, t825: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9826 = 0.29792074959875355558e-1 * t9823 * t9824;
    let t9828 = t165 * t2530;
    let t9829 = t161 * t9828;
    let t9830 = t2685 * t9829;
    let t9831 = t2684 * t9830;
    let t9832 = 0.38342925953920749676e0 * t9831;
    let t9833 = t2465 * t2581;
    let t9834 = t2464 * t9833;
    let t9835 = t2684 * t9834;
    let t9836 = 0.85206502119823888169e-1 * t9835;
    let t9837 = t7416 * t3311;
    let t9838 = 0.38342925953920749676e0 * t9837;
    let t9845 = t2013 * t3296;
    let t9846 = 0.38342925953920749676e0 * t9845;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    (t9826, t9829, t9830, t9832, t9833, t9834, t9836, t9838, t9846, t9847, t9848)
}
