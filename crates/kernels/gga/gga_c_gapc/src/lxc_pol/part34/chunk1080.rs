//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1080/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1080<F: Float>(t11329: F, t9262: F, t27063: F, t3709: F, t26017: F, t19771: F, t3718: F, t436: F, t8775: F, t8776: F, t1912: F, t19652: F, t3717: F, t11509: F, t5633: F, t3144: F, t34409: F) -> (F, F, F, F, F, F, F, F) {
    let t34956 = t11329 * t9262;
    let t34958 = t3709 * t27063;
    let t34960 = t3709 * t26017;
    let t34962 = t3718 * t19771;
    let t34965 = t8775 * t436 * t8776;
    let t34971 = t19652 * t3717 * t1912;
    let t34973 = t11509 * t5633;
    let t34975 = t34409 * t3144;
    (t34956, t34958, t34960, t34962, t34965, t34971, t34973, t34975)
}
