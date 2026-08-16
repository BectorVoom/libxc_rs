//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1202/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1202(t32079: f64, t4382: f64, t986: f64, t6470: f64, t9074: f64, t1063: f64, t30189: f64, t30199: f64, t30207: f64, t32053: f64, t32055: f64, t32057: f64, t32059: f64, t32062: f64, t32066: f64, t32067: f64, t32072: f64, t32074: f64, t32077: f64, t535: f64) -> (f64, f64) {
    let t32080 = 0.23712505529730124666e-2_f64 * t32079;
    let t32081 = t4382 * t986;
    let t32083 = t9074 * t32081 * t6470;
    let t32084 = 0.82993769354055436331e-2_f64 * t32083;
    let t32085 = t30189 - t32053 + t32055 - t32057 + t32059 + t32062 - t30199 - t32066 - 0.56910013271352299198e-1_f64 * t1063 * t535 * t32067 - t32072 - t30207 - t32074 - t32077 - t32080 - t32084;
    (t32081, t32085)
}
