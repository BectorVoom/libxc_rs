//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1119/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1119<F: Float>(t42144: F, t36910: F, t36913: F, t36916: F, t36922: F, t36925: F, t36928: F, t36936: F, t36948: F, t38123: F, t4041: F, t42132: F, t42136: F, t42142: F, t42149: F, t44043: F, t44057: F, t44071: F, t44086: F, t44101: F, t44115: F, t44130: F, t44146: F, t44162: F, t44168: F, t44203: F, t44230: F, t44264: F, t44292: F, t44320: F, t44342: F, t44368: F, t72: F, t739: F, t82: F, t9352: F) -> F {
    let t44382 = F::cast_from(0.49658699875514145965e-4_f64) * t42144;
    let t44384 = F::cast_from(0.1440846329149835838e-2_f64) * t36910 + F::cast_from(0.1440846329149835838e-2_f64) * t36913 + F::cast_from(0.13242319966803772257e-3_f64) * t36916 - F::cast_from(0.76845137554657911361e-2_f64) * t36922 - F::cast_from(0.2881692658299671676e-2_f64) * t36925 - F::cast_from(0.19863479950205658386e-4_f64) * t36928 - F::cast_from(0.1440846329149835838e-2_f64) * t36936 + t38123 + F::cast_from(0.40992351065071538965e-3_f64) * t36948 + t72 * t82 * (t44043 + t44057 + t44071 + t44086 + t44101 + t44115 + t44130 + t44146 + t44168 + t44203 + t44230 + t44264 + t44292 + t44320 + t44342 + t44368) - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t44162 + F::cast_from(0.11974241701863808564e0_f64) * t4041 * t9352 + F::cast_from(0.14546486215597515589e0_f64) * t42132 - F::cast_from(0.15323255961587222184e-3_f64) * t42136 - F::cast_from(0.23942587439980034662e-4_f64) * t42142 - t44382 + F::cast_from(0.3192344991997337955e-4_f64) * t42149;
    t44384
}
