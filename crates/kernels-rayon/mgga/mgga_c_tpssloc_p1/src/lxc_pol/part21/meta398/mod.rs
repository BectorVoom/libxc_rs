//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1874;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta398(t2844: f64, t4395: f64, t912: f64, t2842: f64, t2836: f64, t4399: f64, t10704: f64, t1556: f64, t2793: f64, t10702: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10832: f64, t13563: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14389, t14391, t14392, t14394, t14396, t14398, t14409, t14410) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1874(t2844, t4395, t912, t2842, t2836, t4399, t10704, t1556, t2793, t10702, t13566, t13602);
        let t14419 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1875(t10556, t10558, t10560, t10562, t10832, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613, t14409, t14410);
    (t14389, t14391, t14392, t14394, t14396, t14398, t14409, t14410, t14419)
}
