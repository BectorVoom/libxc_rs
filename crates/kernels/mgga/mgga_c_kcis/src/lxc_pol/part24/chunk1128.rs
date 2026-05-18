//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1128/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1128<F: Float>(t1008: F, t71722: F, t1262: F, t6276: F, t167: F, t1851: F, t26391: F, t26399: F, t26401: F, t26409: F, t26655: F, t26520: F) -> (F, F, F, F, F, F, F, F, F) {
    let t81752 = t71722 * t1008;
    let t84759 = t6276 * t1262;
    let t84812 = t1851 * t167;
    let t91769 = F::new(18.0) * t26391;
    let t91772 = F::new(6.0) * t26399;
    let t91773 = F::new(12.0) * t26401;
    let t91776 = F::new(6.0) * t26409;
    let t91777 = F::new(3.0) * t26655;
    let t91778 = F::new(3.0) * t26520;
    (t81752, t84759, t84812, t91769, t91772, t91773, t91776, t91777, t91778)
}
