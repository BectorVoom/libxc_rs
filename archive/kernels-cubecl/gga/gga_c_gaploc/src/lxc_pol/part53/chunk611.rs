//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 611/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk611<F: Float>(t10743: F, t943: F, t2549: F, t3437: F, t2558: F, t3049: F, t2936: F, t7671: F, t1897: F, t8942: F, t954: F, t3440: F, t7129: F) -> (F, F, F, F, F, F) {
    let t10744 = t943 * t10743;
    let t10745 = F::cast_from(0.32043859292259267849e-3_f64) * t10744;
    let t10746 = t2549 * t3437;
    let t10747 = F::cast_from(0.32043859292259267849e-3_f64) * t10746;
    let t10749 = t3049 * t2558;
    let t10750 = t943 * t10749;
    let t10751 = F::cast_from(0.32043859292259267849e-3_f64) * t10750;
    let t10752 = t2936 * t7671;
    let t10754 = F::cast_from(0.23071578690426672851e-1_f64) * t1897 * t10752;
    let t10755 = t954 * t8942;
    let t10757 = F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t10755;
    let t10759 = F::cast_from(0.23071578690426672851e-1_f64) * t7129 * t3440;
    (t10745, t10747, t10751, t10754, t10757, t10759)
}
