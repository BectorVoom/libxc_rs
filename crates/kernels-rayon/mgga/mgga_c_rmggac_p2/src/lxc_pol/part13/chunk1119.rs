//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1119/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1119(t42144: f64, t36910: f64, t36913: f64, t36916: f64, t36922: f64, t36925: f64, t36928: f64, t36936: f64, t36948: f64, t38123: f64, t4041: f64, t42132: f64, t42136: f64, t42142: f64, t42149: f64, t44043: f64, t44057: f64, t44071: f64, t44086: f64, t44101: f64, t44115: f64, t44130: f64, t44146: f64, t44162: f64, t44168: f64, t44203: f64, t44230: f64, t44264: f64, t44292: f64, t44320: f64, t44342: f64, t44368: f64, t72: f64, t739: f64, t82: f64, t9352: f64) -> f64 {
    let t44382 = 0.49658699875514145965e-4_f64 * t42144;
    let t44384 = 0.1440846329149835838e-2_f64 * t36910 + 0.1440846329149835838e-2_f64 * t36913 + 0.13242319966803772257e-3_f64 * t36916 - 0.76845137554657911361e-2_f64 * t36922 - 0.2881692658299671676e-2_f64 * t36925 - 0.19863479950205658386e-4_f64 * t36928 - 0.1440846329149835838e-2_f64 * t36936 + t38123 + 0.40992351065071538965e-3_f64 * t36948 + t72 * t82 * (t44043 + t44057 + t44071 + t44086 + t44101 + t44115 + t44130 + t44146 + t44168 + t44203 + t44230 + t44264 + t44292 + t44320 + t44342 + t44368) - 0.59871208509319042821e-1_f64 * t739 * t44162 + 0.11974241701863808564e0_f64 * t4041 * t9352 + 0.14546486215597515589e0_f64 * t42132 - 0.15323255961587222184e-3_f64 * t42136 - 0.23942587439980034662e-4_f64 * t42142 - t44382 + 0.3192344991997337955e-4_f64 * t42149;
    t44384
}
