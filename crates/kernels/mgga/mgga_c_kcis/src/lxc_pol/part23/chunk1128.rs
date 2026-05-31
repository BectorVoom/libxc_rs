//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1128/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1128<F: Float>(t1881: F, t8015: F, t27741: F, t12861: F, t1607: F, t4314: F, t4455: F, t779: F, t9274: F, t2531: F, t2537: F, t782: F, t9266: F) -> (F, F, F, F, F, F, F) {
    let t28891 = t1881 * t8015;
    let t28901 = F::cast_from(2.0_f64) * t27741;
    let t30409 = t1607 * t12861;
    let t30424 = t4455 * t4314;
    let t31271 = t779 * t9274;
    let t31274 = t2531 * t2537;
    let t35630 = t9266 * t782;
    (t28891, t28901, t30409, t30424, t31271, t31274, t35630)
}
