//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2007/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2007<F: Float>(t105936: F, t95822: F, t102930: F, t102934: F, t102937: F, t102939: F, t102941: F, t102943: F, t102945: F, t1579: F, t18784: F, t2061: F, t25383: F, t28340: F, t29698: F, t30342: F, t4533: F, t6071: F, t7070: F, t7071: F, t7398: F, t7424: F, t7997: F) -> F {
    let t110236 = t95822 * t105936;
    let t110242 = F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t7997 * t4533 - F::cast_from(0.4336814094102599731e0_f64) * t29698 * t7424 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t28340 * t1579 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t2061 * t18784 - t102930 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t30342 + t102934 - t102937 + t102939 - t102941 + F::cast_from(0.28912093960683998207e-1_f64) * t110236 + t102943 - t102945 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t7398 * t6071;
    t110242
}
