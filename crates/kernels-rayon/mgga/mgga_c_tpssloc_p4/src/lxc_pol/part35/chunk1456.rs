//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1456/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1456(t103413: f64, t103494: f64, t104502: f64, t104635: f64, t109722: f64, t1761: f64, t19232: f64, t19249: f64, t2124: f64, t2155: f64, t27406: f64, t27426: f64, t27830: f64, t29554: f64, t29798: f64, t29812: f64, t29816: f64, t5055: f64, t6244: f64, t7283: f64, t73891: f64, t8061: f64, t8088: f64, t94701: f64) -> f64 {
    let t109809 = -0.82246703342411321826e-2_f64 * t103413 + 6.0_f64 * t27830 * t6244 - 0.3752886611772249944e0_f64 * t109722 * t2124 + 6.0_f64 * t19232 * t8061 - 3.0_f64 * t73891 * t2155 - 3.0_f64 * t19232 * t8088 - 0.82246703342411321826e-2_f64 * t7283 * t27426 * t29812 - 0.16449340668482264365e-1_f64 * t7283 * t27426 * t29816 + 0.13159472534785811492e0_f64 * t27406 * t29554 - 18.0_f64 * t5055 * t29798 + 0.54831135561607547884e-2_f64 * t94701 - 0.16449340668482264365e-1_f64 * t103494 - 3.0_f64 * t19249 * t8088 - 3.0_f64 * t104635 * t1761 + 0.16449340668482264365e-1_f64 * t104502;
    t109809
}
