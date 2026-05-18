//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 565/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk565<F: Float>(t10843: F, t825: F, t10627: F, t701: F, t7585: F, t7584: F, t326: F, t2615: F, t3474: F, t5676: F, t2610: F, t2925: F) -> (F, F, F, F, F, F, F, F) {
    let t10844 = t825 * t10843;
    let t10845 = F::new(0.25561950635947166451e0) * t10844;
    let t10847 = t10627 * t701;
    let t10848 = t7585 * t10847;
    let t10850 = F::new(0.11502877786176224903e2) * t7584 * t10848;
    let t10851 = t326 * t10847;
    let t10853 = F::new(0.46011511144704899612e1) * t2615 * t10851;
    let t10854 = t5676 * t3474;
    let t10855 = F::new(0.14896037479937677779e-1) * t10854;
    let t10856 = t2610 * t2925;
    (t10844, t10845, t10847, t10850, t10853, t10854, t10855, t10856)
}
