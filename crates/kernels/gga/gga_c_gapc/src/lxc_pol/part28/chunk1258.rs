//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1258/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1258<F: Float>(t11523: F, t25842: F, t1743: F, t19511: F, t34090: F, t11329: F, t9262: F, t27063: F, t3709: F, t26017: F, t19771: F, t3718: F) -> (F, F, F, F, F, F) {
    let t34951 = t11523 * t25842;
    let t34954 = t1743 * t34090 * t19511;
    let t34956 = t11329 * t9262;
    let t34958 = t3709 * t27063;
    let t34960 = t3709 * t26017;
    let t34962 = t3718 * t19771;
    (t34951, t34954, t34956, t34958, t34960, t34962)
}
