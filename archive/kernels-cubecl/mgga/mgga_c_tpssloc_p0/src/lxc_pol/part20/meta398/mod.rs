//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1792;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1793;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta398<F: Float>(t13654: F, t913: F, t893: F, t2929: F, t4471: F, t4497: F, t959: F, t2904: F, t952: F, t3216: F, t4696: F, t13550: F, t13563: F, t10296: F, t10298: F, t10302: F, t13566: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13644: F, t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F, t10300: F, t10542: F, t10545: F, t10556: F, t10558: F, t10560: F, t10562: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13655, t13657, t13658, t13659, t13661, t13662, t13663, t13665, t13666, t13675) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1792::<F>(t13654, t913, t893, t2929, t4471, t4497, t959, t2904, t952, t3216, t4696, t13550);
        let (t13679, t13692) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1793::<F>(t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let t13716 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1794::<F>(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10542, t10545, t10556, t10558, t10560, t10562, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t13675, t13679, t13692);
    (t13655, t13657, t13658, t13659, t13661, t13662, t13663, t13665, t13666, t13716)
}
