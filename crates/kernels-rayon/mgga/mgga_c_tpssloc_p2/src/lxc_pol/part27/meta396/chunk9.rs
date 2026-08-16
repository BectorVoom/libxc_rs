//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1628/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1628(t15139: f64, t15162: f64, t15213: f64, t15232: f64, t300: f64, t3411: f64, t4875: f64, t14958: f64, t14963: f64, t14969: f64, t14971: f64, t15038: f64, t15040: f64, t15043: f64, t15046: f64, t15048: f64, t15050: f64, t15053: f64, t15056: f64, t15059: f64, t15063: f64, t15066: f64, t15070: f64) -> (f64, f64, f64) {
    let t15235 = t300 * (t15139 + t15162 + t15213 + t15232);
    let t15237 = 0.23392894490538584828e1_f64 * t3411 * t4875;
    let t15238 = -t14958 + t14963 - t14969 - t14971 - t15038 - t15040 - t15043 + t15046 - t15048 + t15050 - t15053 - t15056 - t15059 + t15063 + t15066 + t15070 + t15235 + t15237;
    (t15235, t15237, t15238)
}
