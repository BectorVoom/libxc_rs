//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1152/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1152(t91828: f64, t91830: f64, t91832: f64, t91835: f64, t91837: f64, t91839: f64, t91841: f64, t91844: f64, t91847: f64, t91850: f64, t91852: f64, t91854: f64, t91857: f64, t91859: f64) -> f64 {
    let t92149 = -0.485625e0_f64 * t91828 + 0.1875e0_f64 * t91830 + 0.225e1_f64 * t91832 + 0.97125e1_f64 * t91835 + 0.2428125e0_f64 * t91837 - 0.1875e0_f64 * t91839 - 0.809375e-1_f64 * t91841 - 0.97125e0_f64 * t91844 + 0.485625e1_f64 * t91847 + 0.485625e0_f64 * t91850 - 0.45e1_f64 * t91852 - 0.19425e1_f64 * t91854 + 0.19425e1_f64 * t91857 + 0.3375e1_f64 * t91859;
    t92149
}
