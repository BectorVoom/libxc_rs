//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2182/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2182<F: Float>(t15030: F, t25319: F, t25391: F, t25392: F, t25426: F, t27199: F, t7053: F, t93206: F, t93207: F, t93210: F, t93224: F, t93226: F, t93228: F, t93231: F, t93349: F, t99274: F, t99277: F, t99287: F, t99289: F) -> F {
    let t99295 = F::cast_from(0.4336814094102599731e0_f64) * t27199 * t25426 + t99274 + F::cast_from(0.26341796731742046394e1_f64) * t7053 * t15030 + t93206 + F::cast_from(0.26020884564615598386e1_f64) * t93349 * t25392 * t99277 - F::cast_from(0.2601984143835408805e-1_f64) * t93207 - t93210 + t93224 - F::cast_from(0.72280234901709995518e-2_f64) * t93226 + F::cast_from(0.12851425765524037203e-1_f64) * t93228 - t99287 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t25392 * t99289 - t93231 - F::cast_from(0.26020884564615598386e1_f64) * t27199 * t25319;
    t99295
}
