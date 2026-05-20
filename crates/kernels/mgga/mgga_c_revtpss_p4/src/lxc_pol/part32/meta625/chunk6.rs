//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1984/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1984<F: Float>(t3999: F, t8085: F, t102397: F, t102634: F, t102636: F, t102661: F, t14224: F, t14230: F, t1903: F, t22387: F, t25930: F, t26282: F, t26304: F, t27868: F, t28899: F, t28911: F, t30252: F, t5728: F, t6896: F, t75047: F, t75051: F, t7511: F, t75305: F, t94705: F, t96546: F, t96549: F) -> F {
    let t109731 = t3999 * t8085;
    let t109756 = t102634 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t102661 * t14224 - F::cast_from(0.65854491829355115987e0_f64) * t7511 * t22387 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t109731 * t14230 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t26304 * t75305 - F::cast_from(0.17347256376410398924e1_f64) * t94705 * t30252 + F::cast_from(0.13170898365871023197e1_f64) * t26282 * t6896 - F::cast_from(0.4818682326780666368e-3_f64) * t102636 + F::cast_from(0.26020884564615598386e1_f64) * t27868 * t102397 * t75047 - F::cast_from(0.26020884564615598386e1_f64) * t27868 * t28911 * t75051 + F::cast_from(0.48186823267806663678e-3_f64) * t96546 + F::cast_from(0.34694512752820797848e1_f64) * t25930 * t28911 * t1903 * t14230 + t96549 + F::cast_from(0.26341796731742046394e1_f64) * t28899 * t5728;
    t109756
}
