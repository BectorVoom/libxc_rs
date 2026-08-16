//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1474;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1475;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta282(t10662: f64, t2844: f64, t10661: f64, t10294: f64, t10544: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10314: f64, t10320: f64, t10323: f64, t10530: f64, t10538: f64, t10547: f64, t10550: f64, t10311: f64, t10318: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t10589: f64, t10591: f64, t10597: f64, t10600: f64, t913: f64, t893: f64, t2840: f64, t891: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10663, t10665, t10675, t10676, t10680) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1474(t10662, t2844, t10661, t10294, t10544, t10296, t10298, t10300, t10302, t10307, t10314, t10320, t10323, t10530, t10538, t10547, t10550);
        let t10695 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1475(t10311, t10318, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t10589, t10591, t10597, t10600);
        let (t10696, t10697, t10699, t10701, t10702) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1476(t10680, t10695, t913, t893, t2840, t891, t275);
    (t10663, t10665, t10675, t10676, t10696, t10697, t10699, t10701, t10702)
}
