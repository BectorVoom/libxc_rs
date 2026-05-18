//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 636/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk636<F: Float>(t3853: F, t512: F, t19: F, t27: F, t521: F, t14: F, t22: F, t583: F, t588: F, t1320: F, t1333: F, t123: F, t520: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3854 = t512 * t3853;
    let t3857 = t19 * t27;
    let t3859 = F::new(20.0) * t3857 * t521;
    let t3860 = t14 * t22;
    let t3862 = F::new(12.0) * t3860 * t521;
    let t3863 = t583 * t588;
    let t3865 = F::new(32.0) * t3863 * t521;
    let t3867 = F::new(8.0) * t1320 * t1333;
    let t3869 = t520 * t123;
    (t3854, t3857, t3859, t3860, t3862, t3863, t3865, t3867, t3869)
}
