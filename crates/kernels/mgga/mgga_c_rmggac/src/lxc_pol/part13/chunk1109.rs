//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1109/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1109<F: Float>(t118: F, t326: F, t35865: F, t35869: F, t35873: F, t35877: F, t35886: F, t35890: F, t37439: F, t41077: F, t41079: F, t41084: F, t43163: F, t43975: F) -> F {
    let t44130 = -t37439 - F::new(0.72732431077987577948e-1) * t35865 - F::new(0.18183107769496894487e-1) * t35869 + F::new(0.54549323308490683461e-1) * t35873 - F::new(0.40002837092893167872e0) * t35877 + F::new(0.36366215538993788974e0) * t35886 + F::new(0.10909864661698136692e0) * t35890 - F::new(0.11974241701863808564e0) * t326 * t43163 + F::new(0.11974241701863808564e0) * t118 * t43975 - F::new(0.17961362552795712846e1) * t41077 + F::new(0.11974241701863808564e0) * t41079 - F::new(0.35922725105591425692e0) * t41084;
    t44130
}
