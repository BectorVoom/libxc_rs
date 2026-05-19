//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 915/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk915<F: Float>(t11455: F, t11479: F, t11482: F, t21268: F, t21270: F, t21273: F, t21275: F, t21278: F, t21281: F, t21283: F, t21286: F, t11409: F, t16046: F, t16050: F, t16052: F, t16127: F, t16129: F, t16146: F, t16292: F, t16301: F, t21186: F, t21188: F, t21190: F, t21193: F, t21229: F, t21234: F, t21237: F, t21240: F, t21243: F, t21246: F, t21249: F, t21424: F) -> F {
    let t21445 = -F::cast_from(0.91983333333333333333e-1_f64) * t11455 - t11479 - t11482 + F::new(0.258925e1) * t21268 + F::new(0.16504875e0) * t21270 + F::new(0.19419375e1) * t21273 - F::new(0.258925e1) * t21275 - F::new(0.1294625e1) * t21278 - F::cast_from(0.412621875e-1_f64) * t21281 + F::new(0.16504875e0) * t21283 + F::new(0.82524375e-1) * t21286;
    let t21447 = -F::new(0.22076e0) * t16127 - F::cast_from(0.18396666666666666667e0_f64) * t16129 - F::cast_from(0.40256666666666666668e0_f64) * t16052 - F::cast_from(0.26837777777777777779e0_f64) * t16046 - t16292 + F::cast_from(0.36793333333333333333e-1_f64) * t16146 + F::cast_from(0.67094444444444444443e-1_f64) * t21186 - F::cast_from(0.20128333333333333333e0_f64) * t21188 + F::cast_from(0.18396666666666666667e-1_f64) * t21190 - F::new(0.301925e0) * t21193 + t21424 - F::new(0.27595e-1) * t21229 - F::cast_from(0.13418888888888888889e0_f64) * t11409 + t16301 - F::cast_from(0.40256666666666666668e0_f64) * t16050 + F::new(0.12077e1) * t21234 - F::cast_from(0.33547222222222222222e0_f64) * t21237 + F::cast_from(0.80513333333333333332e0_f64) * t21240 - F::new(0.181155e1) * t21243 + F::new(0.16557e0) * t21246 - F::cast_from(0.36793333333333333333e-1_f64) * t21249 + t21445;
    t21447
}
