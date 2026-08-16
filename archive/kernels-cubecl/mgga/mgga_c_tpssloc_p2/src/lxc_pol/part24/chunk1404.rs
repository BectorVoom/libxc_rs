//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1404/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1404<F: Float>(t23317: F, t23384: F, t225: F, t23572: F, t10348: F, t1052: F, t1066: F, t11084: F, t1922: F, t1923: F, t1955: F, t23314: F, t23346: F, t23365: F, t23369: F, t23378: F, t23395: F, t23571: F, t23582: F, t23595: F, t3016: F, t3169: F, t3174: F, t3176: F, t3206: F, t349: F, t388: F, t6687: F, t6699: F, t6815: F, t82803: F, t83226: F, t990: F) -> F {
    let t83398 = t23384 * t23317;
    let t83408 = t23572 * t225;
    let t83417 = F::cast_from(2.0_f64) * t1052 * t3174 * t1955 * t11084 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t3016 * t6699 - F::cast_from(0.3752886611772249944e0_f64) * t82803 * t1923 + F::cast_from(3.0_f64) * t990 * t23571 * t388 + F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t23365 * t23395 + F::cast_from(6.0_f64) * t23369 * t3176 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t10348 * t1922 - F::cast_from(0.82246703342411321826e-2_f64) * t83398 + F::cast_from(0.65797362673929057459e-1_f64) * t23346 * t23314 + F::cast_from(6.0_f64) * t3169 * t23378 + F::cast_from(6.0_f64) * t1052 * t3174 * t6815 * t3206 - F::cast_from(3.0_f64) * t83408 * t1066 - F::cast_from(0.29243272299524025538e-1_f64) * t23346 * t23595 + t349 * t83226 * t388 - F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t23582;
    t83417
}
