//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1092/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1092<F: Float>(t25411: F, t93386: F, t25431: F, t2435: F, t25339: F, t10871: F, t11010: F, t25392: F, t25416: F, t2723: F, t7053: F, t7070: F, t92907: F, t93346: F, t93349: F, t93351: F, t93355: F, t93365: F, t93369: F, t93372: F, t93375: F, t93378: F, t93382: F, t93384: F) -> (F,) {
    let t93387 = t25411 * t93386;
    let t93389 = t25431 * t93386;
    let t93391 = t2435 * t25339;
    let t93393 = -0.29272321618148349057e-1 * t93346 + 0.78062653693846795158e1 * t93349 * t25392 * t93351 + 0.26020884564615598386e1 * t7070 * t93355 * t92907 * t10871 - 0.26020884564615598386e1 * t7070 * t25416 * t92907 * t2723 - 0.86736281882051994623e-1 * t93365 - 0.39512695097613069591e1 * t7053 * t11010 + 0.15421710918628844643e0 * t93369 + 0.68549505033305214441e-2 * t93372 + 0.77108554593144223218e-1 * t93375 - 0.10281140612419229763e-1 * t93378 - 0.19514881078765566038e-2 * t93382 - 0.28912093960683998208e-1 * t93384 + 0.77108554593144223218e-1 * t93387 - 0.43368140941025997312e-1 * t93389 + 0.21951497276451705329e-1 * t93391;
    (t93393,)
}
