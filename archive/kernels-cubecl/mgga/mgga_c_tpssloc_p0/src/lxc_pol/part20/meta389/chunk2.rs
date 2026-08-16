//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1764/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1764<F: Float>(t776: F, t868: F, t13110: F, t13112: F, t13114: F, t13117: F, t13118: F, t13121: F, t13122: F, t13125: F, t13129: F, t13132: F, t13135: F, t13136: F, t13137: F, t2379: F, t2522: F, t4307: F, t4310: F, t4314: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F) -> (F, F) {
    let t13487 = t776 * t868;
    let t13491 = -F::cast_from(6.0_f64) * t13487 * t2522 * t4307 + F::cast_from(6.0_f64) * t2379 * t4310 * t4314 + t13110 - t13112 - t13114 + t13117 + t13118 - t13121 - t13122 + t13125 + t13129 + t13132 + t13135 + t13136 + t13137 + t9853 + t9859 - t9894 + t9907 - t9921;
    (t13487, t13491)
}
