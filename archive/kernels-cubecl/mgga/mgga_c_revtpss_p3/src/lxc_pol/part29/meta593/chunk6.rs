//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1983/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1983<F: Float>(t98281: F, t98285: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94565: F, t96358: F, t96359: F) -> F {
    let t102567 = F::cast_from(0.22866142996303859718e-3_f64) * t98281;
    let t102569 = F::cast_from(0.72286371995927450867e-4_f64) * t98285;
    let t102570 = -F::cast_from(0.2032800112371413129e-3_f64) * t94542 - F::cast_from(0.18140473443734395377e0_f64) * t94546 + F::cast_from(0.16006300097412701803e-1_f64) * t94548 - F::cast_from(0.57165357490759649296e-4_f64) * t94552 - F::cast_from(0.6097638132347695925e-3_f64) * t94554 + F::cast_from(0.28582678745379824648e-4_f64) * t94557 - F::cast_from(0.80031500487063509015e-1_f64) * t94559 + F::cast_from(0.10164000561857065645e-2_f64) * t94561 + t102567 - F::cast_from(0.36143185997963725434e-4_f64) * t94565 - t96358 - t96359 - t102569;
    t102570
}
