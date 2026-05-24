//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 350/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk350<F: Float>(t1320: F, t2168: F, t1310: F, t1307: F, t1309: F, t2160: F, t2164: F, t405: F, t1328: F, t1341: F, t2075: F, t1340: F) -> (F, F, F, F, F, F) {
    let t2169 = t1320 * t2168;
    let t2170 = t1310 * t2169;
    let t2173 = F::cast_from(0.5397236614853195164e-1_f64) * t2160 * t405 + t1307 + F::cast_from(0.17990788716177317213e-1_f64) * t1309 * t2164 - F::cast_from(0.5397236614853195164e-1_f64) * t1309 * t2170;
    let t2174 = t2173 * t1328;
    let t2177 = t1341 * t2075;
    let t2178 = t1340 * t2177;
    (t2169, t2170, t2173, t2174, t2177, t2178)
}
