//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 518/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk518<F: Float>(t10847: F, t326: F, t2615: F, t3474: F, t5676: F, t2610: F, t2925: F, t2365: F, t2033: F, t6066: F, t7630: F, t8521: F, t959: F, t2660: F, t8793: F, t787: F, t8792: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10851 = t326 * t10847;
    let t10853 = 0.46011511144704899612e1 * t2615 * t10851;
    let t10854 = t5676 * t3474;
    let t10855 = 0.14896037479937677779e-1 * t10854;
    let t10856 = t2610 * t2925;
    let t10857 = t2365 * t10856;
    let t10858 = t2033 * t10857;
    let t10859 = 0.14896037479937677779e-1 * t10858;
    let t10860 = t6066 * t10847;
    let t10862 = 0.71500979903700853338e0 * t7630 * t10860;
    let t10863 = t8521 * t959;
    let t10864 = 0.14896037479937677779e-1 * t10863;
    let t10866 = 0.10725146985555128001e1 * t8793 * t2660;
    let t10867 = t787 * t8792;
    (t10853, t10854, t10855, t10858, t10859, t10862, t10863, t10864, t10866, t10867)
}
