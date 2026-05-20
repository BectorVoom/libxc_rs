//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2108/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2108<F: Float>(t13951: F, t2018: F, t807: F, t94565: F, t25240: F, t3964: F, t5617: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94569: F, t94571: F) -> F {
    let t98281 = t807 * t2018 * t13951;
    let t98282 = F::cast_from(0.11433071498151929859e-3_f64) * t98281;
    let t98283 = F::cast_from(0.18071592998981862717e-4_f64) * t94565;
    let t98285 = t3964 * t25240 * t5617;
    let t98287 = -F::cast_from(0.10164000561857065645e-3_f64) * t94542 - F::cast_from(0.90702367218671976886e-1_f64) * t94546 + F::cast_from(0.80031500487063509016e-2_f64) * t94548 - F::cast_from(0.28582678745379824648e-4_f64) * t94552 - F::cast_from(0.30488190661738479624e-3_f64) * t94554 + F::cast_from(0.14291339372689912324e-4_f64) * t94557 - F::cast_from(0.40015750243531754508e-1_f64) * t94559 + F::cast_from(0.50820002809285328225e-3_f64) * t94561 + t98282 - t98283 - t94569 - t94571 - F::cast_from(0.36143185997963725434e-4_f64) * t98285;
    t98287
}
