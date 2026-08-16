//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 783/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk783<F: Float>(t2222: F, t3517: F, t2188: F, t3598: F, t2226: F, t11313: F, t2218: F, t1354: F, t2083: F, t2079: F, t3676: F, t2089: F, t2877: F) -> (F, F, F, F, F, F, F) {
    let t19163 = t3517 * t2222;
    let t19182 = t3598 * t2188;
    let t19235 = t3517 * t2226;
    let t19404 = t11313 * t2218;
    let t19434 = t1354 * t2083;
    let t19476 = t2079 * t3676;
    let t19543 = t2877 * t2089;
    (t19163, t19182, t19235, t19404, t19434, t19476, t19543)
}
