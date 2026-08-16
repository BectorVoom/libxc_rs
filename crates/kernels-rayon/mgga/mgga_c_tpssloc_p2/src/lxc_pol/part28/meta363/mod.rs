//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1358;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1359;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta363(t13602: f64, t12606: f64, t883: f64, t882: f64, t123: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10577: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13600: f64, t13601: f64, t901: f64, t2815: f64, t4370: f64, t896: f64, t2807: f64, t4378: f64, t2798: f64, t4362: f64, t10595: f64, t1547: f64, t2799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13603, t13611, t13613) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1358(t13602, t12606, t883, t882, t123);
        let t13615 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1359(t10556, t10558, t10560, t10562, t10577, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13600, t13601, t13603, t13613);
        let (t13616, t13624, t13626, t13630, t13632, t13635) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1360(t13615, t901, t2815, t4370, t896, t2807, t4378, t2798, t4362, t10595, t1547, t2799);
    (t13611, t13613, t13615, t13616, t13624, t13626, t13630, t13632, t13635)
}
