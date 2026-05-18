//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1208/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1208<F: Float>(t21556: F, t3440: F, t2554: F, t7064: F, t8871: F, t1897: F, t7671: F, t8637: F, t3437: F, t7211: F, t10749: F, t2549: F) -> (F, F, F, F, F) {
    let t32400 = F::new(0.6152420984113779427e-1) * t21556 * t3440;
    let t32407 = t7064 * t8871 * t2554;
    let t32408 = F::new(0.64087718584518535698e-3) * t32407;
    let t32411 = F::new(0.46143157380853345702e-1) * t1897 * t8637 * t7671;
    let t32412 = t7211 * t3437;
    let t32413 = F::new(0.32043859292259267849e-3) * t32412;
    let t32414 = t2549 * t10749;
    (t32400, t32408, t32411, t32413, t32414)
}
