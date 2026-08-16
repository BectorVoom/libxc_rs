//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1439/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1439(t103581: f64, t103687: f64, t103710: f64, t103723: f64, t103767: f64, t103774: f64, t1244: f64, t1246: f64, t1653: f64, t1734: f64, t22368: f64, t24812: f64, t24815: f64, t24849: f64, t27507: f64, t29664: f64, t29716: f64, t29727: f64, t29749: f64, t29753: f64, t29754: f64, t3610: f64, t7373: f64, t7376: f64, t7999: f64, t8073: f64, t8074: f64, t8082: f64, t86037: f64, t94858: f64, t94932: f64, t94936: f64, t94966: f64) -> f64 {
    let t109244 = 0.65797362673929057459e-1_f64 * t94858 * t29754 + 0.24125699647107321069e0_f64 * t103581 * t8074 + 3.0_f64 * t1244 * t29664 * t1734 * t1246 - 0.49348022005446793095e-1_f64 * t24812 * t94932 * t29749 - 0.24674011002723396548e-1_f64 * t7373 * t103687 * t8073 - 0.24674011002723396548e-1_f64 * t7373 * t103723 * t8073 + 0.24674011002723396548e-1_f64 * t24812 * t94936 * t29753 - 0.16449340668482264365e-1_f64 * t86037 * t103774 * t24815 * t1653 - 0.82246703342411321826e-2_f64 * t24849 * t103767 * t7376 * t1653 + 0.18277045187202515961e-2_f64 * t94966 - 0.54831135561607547883e-2_f64 * t103710 + 0.13159472534785811492e0_f64 * t27507 * t29716 - 0.65797362673929057459e-1_f64 * t7999 * t29727 + 6.0_f64 * t3610 * t8082 * t22368;
    t109244
}
