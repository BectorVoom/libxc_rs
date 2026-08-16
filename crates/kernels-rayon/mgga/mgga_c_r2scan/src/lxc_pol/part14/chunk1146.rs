//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1146/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1146(t3597: f64, t6182: f64, t11711: f64, t6425: f64, t1055: f64, t7918: f64, t24063: f64, t24064: f64, t3332: f64, t10868: f64, t7628: f64, t7629: f64) -> (f64, f64, f64, f64, f64) {
    let t39945 = t6182 * t3597;
    let t39947 = t6425 * t11711;
    let t39951 = t7918 * t1055;
    let t39954 = t24063 * t3332 * t24064;
    let t39958 = t7628 * t10868 * t7629;
    (t39945, t39947, t39951, t39954, t39958)
}
