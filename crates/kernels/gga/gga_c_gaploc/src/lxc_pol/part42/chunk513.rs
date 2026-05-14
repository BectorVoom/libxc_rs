//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 513/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk513<F: Float>(t10743: F, t943: F, t2549: F, t3437: F, t2558: F, t3049: F, t2936: F, t7671: F, t1897: F, t8942: F, t954: F, t3440: F, t7129: F, t8637: F, t948: F, t2508: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10744 = t943 * t10743;
    let t10745 = 0.32043859292259267849e-3 * t10744;
    let t10746 = t2549 * t3437;
    let t10747 = 0.32043859292259267849e-3 * t10746;
    let t10749 = t3049 * t2558;
    let t10750 = t943 * t10749;
    let t10751 = 0.32043859292259267849e-3 * t10750;
    let t10752 = t2936 * t7671;
    let t10754 = 0.23071578690426672851e-1 * t1897 * t10752;
    let t10755 = t954 * t8942;
    let t10757 = 0.76905262301422242837e-2 * t1897 * t10755;
    let t10759 = 0.23071578690426672851e-1 * t7129 * t3440;
    let t10760 = t8637 * t948;
    let t10762 = 0.23071578690426672851e-1 * t2508 * t10760;
    (t10744, t10745, t10746, t10747, t10750, t10751, t10754, t10757, t10759, t10762)
}
