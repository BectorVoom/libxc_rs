//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1265/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1265<F: Float>(t10871: F, t11010: F, t25392: F, t25416: F, t2723: F, t7053: F, t7070: F, t92907: F, t93346: F, t93349: F, t93351: F, t93355: F, t93365: F, t93369: F, t93372: F, t93375: F, t93378: F, t93382: F, t93384: F, t93387: F, t93389: F, t93391: F) -> F {
    let t93393 = -F::cast_from(0.29272321618148349057e-1_f64) * t93346 + F::cast_from(0.78062653693846795158e1_f64) * t93349 * t25392 * t93351 + F::cast_from(0.26020884564615598386e1_f64) * t7070 * t93355 * t92907 * t10871 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25416 * t92907 * t2723 - F::cast_from(0.86736281882051994623e-1_f64) * t93365 - F::cast_from(0.39512695097613069591e1_f64) * t7053 * t11010 + F::cast_from(0.15421710918628844643e0_f64) * t93369 + F::cast_from(0.68549505033305214441e-2_f64) * t93372 + F::cast_from(0.77108554593144223218e-1_f64) * t93375 - F::cast_from(0.10281140612419229763e-1_f64) * t93378 - F::cast_from(0.19514881078765566038e-2_f64) * t93382 - F::cast_from(0.28912093960683998208e-1_f64) * t93384 + F::cast_from(0.77108554593144223218e-1_f64) * t93387 - F::cast_from(0.43368140941025997312e-1_f64) * t93389 + F::cast_from(0.21951497276451705329e-1_f64) * t93391;
    t93393
}
