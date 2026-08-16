//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2120/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2120<F: Float>(t6017: F, t886: F, t29668: F, t689: F, t25431: F, t25411: F, t14495: F, t25391: F, t25392: F, t27189: F, t27312: F, t27349: F, t27353: F, t4487: F, t93151: F, t93158: F, t93161: F, t99186: F, t99188: F, t99202: F, t99206: F, t99334: F) -> F {
    let t106143 = t6017 * t886;
    let t106150 = t29668 * t689;
    let t106151 = t25431 * t106150;
    let t106153 = t25411 * t106150;
    let t106164 = -F::cast_from(0.17347256376410398924e1_f64) * t25391 * t99334 * t27312 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t99334 * t14495 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t25392 * t106143 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t99334 * t27349 - F::cast_from(0.14456046980341999104e-1_f64) * t106151 + F::cast_from(0.25702851531048074406e-1_f64) * t106153 - F::cast_from(0.96373646535613327357e-2_f64) * t93151 + F::cast_from(0.26019841438354088051e-1_f64) * t99186 + F::cast_from(0.14634331517634470219e-1_f64) * t99188 + F::cast_from(0.17135234354032049604e-2_f64) * t93158 - F::cast_from(0.22849835011101738147e-2_f64) * t93161 + F::cast_from(0.34270468708064099208e-1_f64) * t99202 + F::cast_from(0.26341796731742046394e1_f64) * t27189 * t4487 - F::cast_from(0.4818682326780666368e-3_f64) * t99206;
    t106164
}
