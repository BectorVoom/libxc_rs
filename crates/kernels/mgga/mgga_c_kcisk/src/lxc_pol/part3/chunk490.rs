//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 490/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk490<F: Float>(t213: F, t442: F, t1390: F, t967: F, t1056: F, t1399: F, t970: F, t1398: F, t3583: F, t1349: F, t1391: F, t173: F, t3283: F, t3844: F, t3848: F, t3851: F, t3852: F, t3853: F) -> (F, F, F) {
    let t3857 = t213 * t442;
    let t3858 = F::cast_from(0.15538616723388920628e-3_f64) * t3857;
    let t3859 = t967 * t1390;
    let t3860 = t3859 * t1056;
    let t3864 = t970 * t1399;
    let t3866 = t1398 * t3583;
    let t3869 = -t3844 - t3848 + t3851 - t3852 - F::cast_from(0.23911438650126355246e-1_f64) * t3853 + F::cast_from(0.11955719325063177623e-1_f64) * t1349 * t3283 + t3858 + F::cast_from(0.20718155631185227504e-3_f64) * t3860 - F::cast_from(0.5179538907796306876e-4_f64) * t1391 * t3283 - F::new(0.23526125e-4) * t3864 + F::new(0.50413125e-5) * t173 * t3866;
    (t3859, t3866, t3869)
}
