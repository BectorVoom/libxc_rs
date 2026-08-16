//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1192/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1192(t2832: f64, t605: f64, t2408: f64, t2411: f64, t14365: f64, t1940: f64, t2257: f64, t2403: f64, t25206: f64, t25211: f64, t25436: f64, t25445: f64, t27158: f64, t27382: f64, t7087: f64, t7091: f64, t7092: f64, t92742: f64, t92743: f64, t92747: f64, t92753: f64, t92759: f64, t92762: f64, t92765: f64, t92768: f64, t92772: f64, t92775: f64) -> f64 {
    let t92779 = t605 * t2832;
    let t92783 = t605 * t2408;
    let t92790 = t2411 * t605;
    let t92791 = t92790 * t14365;
    let t92794 = -3.0_f64 * t1940 * t92742 * t92743 + 9.0_f64 * t25206 * t92747 + 9.0_f64 * t2403 * t7087 * t25211 - 9.0_f64 * t27158 * t92753 + 3.0_f64 / 2.0_f64 * t1940 * t7087 * t2257 - 9.0_f64 / 2.0_f64 * t25206 * t92759 + 3.0_f64 * t27382 * t92762 - 9.0_f64 / 2.0_f64 * t25206 * t92765 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t92768 + 9.0_f64 * t27158 * t92772 - 3.0_f64 / 2.0_f64 * t1940 * t92775 * t7092 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t92779 + 3.0_f64 * t1940 * t25445 * t92783 + 3.0_f64 / 2.0_f64 * t1940 * t25436 * t605 - 9.0_f64 * t25206 * t92791;
    t92794
}
