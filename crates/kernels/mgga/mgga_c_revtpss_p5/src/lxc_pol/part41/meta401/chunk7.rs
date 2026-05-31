//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1381/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1381<F: Float>(t21040: F, t3629: F, t3626: F, t12840: F, t20795: F, t1222: F, t1227: F, t13012: F, t17593: F, t17619: F, t17622: F, t21200: F, t21203: F, t21210: F, t21213: F, t21216: F, t3625: F, t5340: F, t5369: F, t5373: F, t5384: F, t5386: F) -> F {
    let t21218 = t21040 * t3629;
    let t21219 = t3626 * t21218;
    let t21222 = t20795 * t12840;
    let t21223 = t3626 * t21222;
    let t21226 = t17593 + F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t21200 - F::cast_from(0.45732285992607719436e-2_f64) * t21203 * t5386 + t13012 / F::cast_from(1296.0_f64) - t17619 - t17622 + t5373 * t5369 / F::cast_from(54.0_f64) - t1222 * t21210 / F::cast_from(288.0_f64) - F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t21213 * t1227 - F::cast_from(0.19055119163586549765e-3_f64) * t21216 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t21219 - F::cast_from(0.28582678745379824648e-3_f64) * t5340 * t21223;
    t21226
}
