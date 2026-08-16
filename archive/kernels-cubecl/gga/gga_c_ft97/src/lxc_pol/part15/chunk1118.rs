//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1118/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1118<F: Float>(t4960: F, t5049: F, t13582: F, t17824: F, t17836: F, t17851: F, t17870: F, t18090: F, t21134: F, t21144: F, t21145: F, t21277: F, t21281: F, t21386: F, t2379: F, t238: F, t2387: F, t2394: F, t4950: F, t4951: F, t4961: F, t4982: F, t4987: F, t5016: F, t66578: F, t678: F, t79439: F, t80003: F, t807: F, t88413: F, t88433: F, t88444: F, t88447: F, t88456: F, t88462: F, t9533: F) -> F {
    let t88470 = t4960 * t5049;
    let t88480 = F::cast_from(0.82704389902445944776e-3_f64) * t17870 * t21386 * t21134 - F::cast_from(0.16540877980489188956e-2_f64) * t21144 * t21277 * t21145 - F::cast_from(0.16540877980489188956e-2_f64) * t21144 * t21281 * t21145 + F::cast_from(0.23238868087529279928e-2_f64) * t18090 * t4982 + F::cast_from(0.82704389902445944777e-3_f64) * t17870 * t4950 * t4951 * t79439 + F::cast_from(0.93019603785751168e-2_f64) * t678 * t2394 * t88433 - F::cast_from(0.16223712540858999423e-2_f64) * t17851 * t66578 + F::cast_from(0.60826526699468500834e-9_f64) * t238 * t88444 + F::cast_from(0.23238868087529279928e-2_f64) * t9533 * t2379 * t88447 + F::cast_from(0.16864243845320605903e-2_f64) * t4987 * t5016 - F::cast_from(0.22941158433316392859e1_f64) * t238 * t88456 + F::cast_from(0.279058811357253504e-1_f64) * t9533 * t2394 * t88447 - F::cast_from(0.19352371901929178119e-4_f64) * t678 * t807 * t88462 + F::cast_from(0.279058811357253504e-1_f64) * t18090 * t4961 - F::cast_from(0.27020878774141382658e-4_f64) * t80003 * t13582 - F::cast_from(0.139529405678626752e-1_f64) * t2387 * t2394 * t88470 - F::cast_from(0.11619434043764639964e-2_f64) * t2387 * t2379 * t88470 + F::cast_from(0.33081755960978377911e-3_f64) * t17836 * t17824 * t88413;
    t88480
}
