//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2123/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2123<F: Float>(t27216: F, t27279: F, t27213: F, t6022: F, t886: F, t29674: F, t689: F, t25431: F, t25411: F, t14587: F, t18324: F, t18615: F, t1949: F, t231: F, t25322: F, t25391: F, t2718: F, t27267: F, t27353: F, t27357: F, t6072: F, t7053: F, t7070: F, t7076: F, t7759: F, t7766: F, t93206: F, t93207: F, t93210: F, t93224: F, t99274: F) -> F {
    let t106216 = t27216 * t27279;
    let t106218 = t27213 * t27279;
    let t106228 = t6022 * t886;
    let t106235 = t29674 * t689;
    let t106236 = t25431 * t106235;
    let t106238 = t25411 * t106235;
    let t106245 = -F::cast_from(0.25702851531048074406e-1_f64) * t106216 + F::cast_from(0.14456046980341999104e-1_f64) * t106218 + t99274 - F::cast_from(0.8673628188205199462e0_f64) * t7766 * t27267 + F::cast_from(0.13170898365871023197e1_f64) * t7053 * t18324 + t93206 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t2718 * t7759 * t14587 + F::cast_from(0.17347256376410398924e1_f64) * t25391 * t27357 * t106228 - F::cast_from(0.65854491829355115987e0_f64) * t25322 * t6072 - F::cast_from(0.13009920719177044025e-1_f64) * t93207 - t93210 + t93224 - F::cast_from(0.72280234901709995518e-2_f64) * t106236 + F::cast_from(0.12851425765524037203e-1_f64) * t106238 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t1949 * t18615 * t231;
    t106245
}
