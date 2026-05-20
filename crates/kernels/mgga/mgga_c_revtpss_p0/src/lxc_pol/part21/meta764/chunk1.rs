//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2713/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2713<F: Float>(t1522: F, t40158: F, t14362: F, t9575: F, t123: F, t2630: F, t4392: F, t4398: F, t9318: F, t11231: F, t14330: F, t4402: F) -> (F, F, F, F, F) {
    let t49925 = F::new(4.0) * t40158 * t1522;
    let t49926 = t14362 * t9575;
    let t49927 = F::cast_from(0.21687162600603479684e-1_f64) * t49926;
    let t49929 = t4392 * t123 * t2630;
    let t49930 = F::cast_from(0.32530743900905219526e-1_f64) * t49929;
    let t49940 = t4398 * t9318;
    let t49941 = F::cast_from(0.35089341735807877242e1_f64) * t49940;
    let t49944 = F::new(72.0) * t14330 * t4402 * t11231;
    (t49925, t49927, t49930, t49941, t49944)
}
