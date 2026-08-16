//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1192/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1192<F: Float>(t2832: F, t605: F, t2408: F, t2411: F, t14365: F, t1940: F, t2257: F, t2403: F, t25206: F, t25211: F, t25436: F, t25445: F, t27158: F, t27382: F, t7087: F, t7091: F, t7092: F, t92742: F, t92743: F, t92747: F, t92753: F, t92759: F, t92762: F, t92765: F, t92768: F, t92772: F, t92775: F) -> F {
    let t92779 = t605 * t2832;
    let t92783 = t605 * t2408;
    let t92790 = t2411 * t605;
    let t92791 = t92790 * t14365;
    let t92794 = -F::cast_from(3.0_f64) * t1940 * t92742 * t92743 + F::cast_from(9.0_f64) * t25206 * t92747 + F::cast_from(9.0_f64) * t2403 * t7087 * t25211 - F::cast_from(9.0_f64) * t27158 * t92753 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7087 * t2257 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t25206 * t92759 + F::cast_from(3.0_f64) * t27382 * t92762 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t25206 * t92765 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7091 * t92768 + F::cast_from(9.0_f64) * t27158 * t92772 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t92775 * t7092 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7091 * t92779 + F::cast_from(3.0_f64) * t1940 * t25445 * t92783 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t25436 * t605 - F::cast_from(9.0_f64) * t25206 * t92791;
    t92794
}
