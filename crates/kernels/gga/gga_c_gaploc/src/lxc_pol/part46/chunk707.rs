//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 707/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk707<F: Float>(t13023: F, t1457: F, t2103: F, t3040: F, t3271: F, t11001: F, t955: F, t10948: F, t3470: F, t3209: F, t8604: F, t1445: F) -> (F, F, F, F, F, F, F) {
    let t13024 = t1457 * t13023;
    let t13026 = F::new(0.71500979903700853338e0) * t2103 * t13024;
    let t13028 = F::new(0.35750489951850426669e0) * t3271 * t3040;
    let t13029 = t955 * t11001;
    let t13031 = t10948 * t3470;
    let t13033 = t8604 * t3209;
    let t13034 = t1445 * t13033;
    (t13024, t13026, t13028, t13029, t13031, t13033, t13034)
}
