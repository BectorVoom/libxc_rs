//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2496/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2496<F: Float>(t49476: F, t1358: F, t2439: F, t5710: F, t785: F, t1426: F, t5711: F, t786: F, t14100: F, t9686: F, t1353: F, t198: F) -> (F, F, F, F, F) {
    let t49477 = F::cast_from(0.21951497276451705329e-1_f64) * t49476;
    let t49480 = t2439 * t785 * t5710 * t1358;
    let t49503 = t786 * t5711 * t1426;
    let t49512 = t14100 * t9686;
    let t49513 = F::cast_from(0.39029762157531132076e-1_f64) * t49512;
    let t49541 = t198 * t1353;
    (t49477, t49480, t49503, t49513, t49541)
}
