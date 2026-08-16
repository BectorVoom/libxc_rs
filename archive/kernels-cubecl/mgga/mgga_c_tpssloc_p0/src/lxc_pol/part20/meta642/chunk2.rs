//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2352/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2352<F: Float>(t10186: F, t10208: F, t10237: F, t10245: F, t13769: F, t13791: F, t13794: F, t13798: F, t13851: F, t23547: F, t2771: F, t2986: F, t2990: F, t340: F, t343: F, t42799: F, t42830: F, t43071: F, t4510: F, t4531: F, t4532: F, t47679: F, t47697: F, t47742: F, t48120: F, t48169: F, t48180: F, t48184: F, t48189: F, t48191: F, t48207: F, t48210: F, t48215: F, t48217: F, t48221: F, t48233: F, t6733: F, t884: F, t973: F, t974: F) -> F {
    let t48235 = -F::cast_from(0.59259259259259259257e-2_f64) * t10186 * t13791 - F::cast_from(0.29629629629629629629e-2_f64) * t10186 * t13794 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t974 * t340 * (t48120 + t48169) * t343 + F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t4510 * t47697 + F::cast_from(0.28806584362139917695e-2_f64) * t2986 * t48180 * t47679 - F::cast_from(0.24999999999999999999e-2_f64) * t2986 * t48184 * t10208 - F::cast_from(0.27777777777777777777e-3_f64) * t48189 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t48191 * t2990 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t13851 * t10245 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t13769 * t6733 * t2771 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t4531 * t23547 * t884 - F::cast_from(0.83333333333333333331e-3_f64) * t48207 + F::cast_from(0.37037037037037037036e-3_f64) * t48210 + F::cast_from(0.86419753086419753084e-3_f64) * t48215 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t48217 * t10237 - F::cast_from(0.86419753086419753084e-3_f64) * t2986 * t48221 * t43071 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t4531 * t42799 - F::cast_from(0.1037037037037037037e-1_f64) * t2986 * t13798 * t47742 - F::cast_from(0.81481481481481481478e-2_f64) * t42830 * t4532 + F::cast_from(0.14814814814814814814e-2_f64) * t48233;
    t48235
}
