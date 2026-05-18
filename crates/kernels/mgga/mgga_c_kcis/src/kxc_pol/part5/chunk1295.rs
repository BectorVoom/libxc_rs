//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1295/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1295<F: Float>(t11455: F, t11479: F, t11482: F, t21268: F, t21270: F, t21273: F, t21275: F, t21278: F, t21281: F, t21283: F, t21286: F, t11409: F, t16046: F, t16050: F, t16052: F, t16127: F, t16129: F, t16146: F, t16292: F, t16301: F, t21186: F, t21188: F, t21190: F, t21193: F, t21229: F, t21234: F, t21237: F, t21240: F, t21243: F, t21246: F, t21249: F, t21424: F) -> F {
    let t21445 = -F::new(0.91983333333333333333e-1) * t11455 - t11479 - t11482 + F::new(0.258925e1) * t21268 + F::new(0.16504875e0) * t21270 + F::new(0.19419375e1) * t21273 - F::new(0.258925e1) * t21275 - F::new(0.1294625e1) * t21278 - F::new(0.412621875e-1) * t21281 + F::new(0.16504875e0) * t21283 + F::new(0.82524375e-1) * t21286;
    let t21447 = -F::new(0.22076e0) * t16127 - F::new(0.18396666666666666667e0) * t16129 - F::new(0.40256666666666666668e0) * t16052 - F::new(0.26837777777777777779e0) * t16046 - t16292 + F::new(0.36793333333333333333e-1) * t16146 + F::new(0.67094444444444444443e-1) * t21186 - F::new(0.20128333333333333333e0) * t21188 + F::new(0.18396666666666666667e-1) * t21190 - F::new(0.301925e0) * t21193 + t21424 - F::new(0.27595e-1) * t21229 - F::new(0.13418888888888888889e0) * t11409 + t16301 - F::new(0.40256666666666666668e0) * t16050 + F::new(0.12077e1) * t21234 - F::new(0.33547222222222222222e0) * t21237 + F::new(0.80513333333333333332e0) * t21240 - F::new(0.181155e1) * t21243 + F::new(0.16557e0) * t21246 - F::new(0.36793333333333333333e-1) * t21249 + t21445;
    t21447
}
