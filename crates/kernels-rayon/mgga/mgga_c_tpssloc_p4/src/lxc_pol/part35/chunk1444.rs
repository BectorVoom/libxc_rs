//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1444/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1444(t6224: f64, t8054: f64, t103959: f64, t109385: f64, t11881: f64, t11883: f64, t11888: f64, t11889: f64, t15027: f64, t2121: f64, t2147: f64, t21769: f64, t21776: f64, t22327: f64, t27406: f64, t29705: f64, t29712: f64, t29723: f64, t29790: f64, t3624: f64, t3625: f64, t462: f64, t5064: f64, t6140: f64, t6168: f64, t7283: f64, t7362: f64, t7363: f64, t8077: f64, t8085: f64, t95726: f64) -> (f64, f64) {
    let t109418 = t8054 * t6224;
    let t109432 = 3.0_f64 * t6168 * t8085 + 0.14621636149762012769e-1_f64 * t103959 - 0.27415567780803773942e-2_f64 * t7283 * t7362 * t7363 * t21776 - 0.54831135561607547884e-2_f64 * t95726 + 6.0_f64 * t15027 * t29723 + 3.0_f64 * t5064 * t29712 - 0.16449340668482264365e-1_f64 * t7283 * t7362 * t7363 * t21769 - 0.24674011002723396548e-1_f64 * t7283 * t6140 * t8077 + 0.82246703342411321825e-2_f64 * t2121 * t462 * t2147 * t22327 - 3.0_f64 * t3624 * t109418 * t3625 - 6.0_f64 * t11888 * t109385 * t11889 + 0.65797362673929057459e-1_f64 * t27406 * t29705 + 0.13159472534785811492e0_f64 * t27406 * t29790 + 6.0_f64 * t11881 * t109385 * t11883;
    (t109418, t109432)
}
