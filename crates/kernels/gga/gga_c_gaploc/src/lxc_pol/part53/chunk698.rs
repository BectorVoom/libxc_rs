//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 698/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk698<F: Float>(t3009: F, t3234: F, t1445: F, t2087: F, t1645: F, t3255: F, t3025: F, t2958: F) -> (F, F, F, F, F, F) {
    let t13012 = t3009 * t3234;
    let t13013 = t1445 * t13012;
    let t13015 = F::new(0.69017266717057349418e1) * t2087 * t13013;
    let t13016 = t1645 * t3255;
    let t13018 = F::new(0.10725146985555128001e1) * t3025 * t13016;
    let t13023 = t2958 * t3234;
    (t13012, t13013, t13015, t13016, t13018, t13023)
}
