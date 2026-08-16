//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1442/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1442(t103218: f64, t103699: f64, t103879: f64, t103927: f64, t15245: f64, t1734: f64, t19201: f64, t2148: f64, t22034: f64, t22040: f64, t24849: f64, t24851: f64, t27406: f64, t29702: f64, t29709: f64, t29750: f64, t29787: f64, t5398: f64, t7283: f64, t7376: f64, t8067: f64, t8070: f64, t8083: f64, t86000: f64, t94395: f64, t94858: f64, t94963: f64) -> f64 {
    let t109356 = -0.82246703342411321826e-2_f64 * t103879 + t86000 - 0.24125699647107321069e0_f64 * t103218 * t8070 - 0.82246703342411321825e-2_f64 * t7283 * t22040 * t2148 - 3.0_f64 * t15245 * t29709 + 3.0_f64 * t19201 * t8083 - 0.82246703342411321826e-2_f64 * t24849 * t24851 * t5398 * t1734 * t7376 + 0.16449340668482264365e-1_f64 * t94963 * t103699 - 0.80418998823691070229e-1_f64 * t103218 * t8067 - 0.13159472534785811492e0_f64 * t94858 * t29750 - 0.43864908449286038307e-1_f64 * t94395 * t29787 + 0.36554090374405031922e-2_f64 * t103927 + 0.65797362673929057459e-1_f64 * t27406 * t29702 - 0.82246703342411321825e-2_f64 * t7283 * t22034 * t2148;
    t109356
}
