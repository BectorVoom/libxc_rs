//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 556/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk556<F: Float>(t2722: F, t2724: F, t2620: F, t331: F, t2246: F, t329: F, t155: F, t889: F, t947: F, t146: F, t2341: F, t318: F) -> (F, F, F, F, F, F) {
    let t2725 = t2722 * t2724;
    let t2729 = F::new(0.16793568152788065763e-2) * t331 * t2620;
    let t2730 = t329 * t2246;
    let t2731 = t155 * t2730;
    let t2734 = t947 * t889;
    let t2737 = t146 * t318 * t2341;
    (t2725, t2729, t2730, t2731, t2734, t2737)
}
