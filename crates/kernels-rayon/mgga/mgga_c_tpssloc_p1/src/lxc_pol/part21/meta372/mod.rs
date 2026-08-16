//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1820;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta372(t13654: f64, t913: f64, t893: f64, t2929: f64, t4471: f64, t4497: f64, t959: f64, t2904: f64, t952: f64, t3216: f64, t4696: f64, t13550: f64, t13563: f64, t10296: f64, t10298: f64, t10302: f64, t13566: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13644: f64, t13602: f64, t13598: f64, t13613: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64, t13647: f64, t10300: f64, t10542: f64, t10545: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13552: f64, t13557: f64, t13561: f64, t13616: f64, t13624: f64, t13626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13655, t13657, t13659, t13661, t13663, t13665, t13666, t13675) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1819(t13654, t913, t893, t2929, t4471, t4497, t959, t2904, t952, t3216, t4696, t13550);
        let (t13679, t13692) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1820(t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let (t13709, t13712, t13716) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1821(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10542, t10545, t10556, t10558, t10560, t10562, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t13675, t13679, t13692);
    (t13655, t13657, t13659, t13661, t13663, t13665, t13666, t13675, t13679, t13709, t13712, t13716)
}
