//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 740/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk740<F: Float>(t21249: F, t290: F, t21253: F, t5272: F, t7853: F, t1701: F, t3780: F, t5295: F, t1208: F, t2035: F, t5266: F, t4064: F, t10373: F, t13643: F, t13648: F, t18032: F, t18035: F, t18038: F, t18040: F, t18081: F, t21184: F, t21188: F, t21190: F, t21194: F, t21198: F, t21202: F, t21206: F, t21213: F, t21216: F, t21218: F, t21220: F) -> (F, F, F, F, F, F, F) {
    let t21999 = t290 * t21249;
    let t22000 = t21999 * t21253;
    let t22003 = t7853 * t5272;
    let t22007 = t1701 * t3780 * t5295;
    let t22013 = t2035 * t5266 * t1208;
    let t22020 = t4064 * t5295;
    let t22059 = 0.48897200801234567904e0 * t18081 + 0.16669500273148148149e-1 * t18032 + 0.22226000364197530866e-1 * t18035 - 0.33339000546296296299e-1 * t18038 - 0.88904001456790123462e-1 * t18040 + 0.51860667516460905352e-1 * t21184 + 0.16669500273148148149e-1 * t21188 + 0.26671200437037037038e0 * t21190 - 0.13335600218518518519e0 * t21194 + 0.66678001092592592595e-1 * t21198 + 0.10001700163888888889e0 * t21202 - 0.10001700163888888889e0 * t21206 + 0.88904001456790123462e-1 * t13643 + t10373 - 0.22818693707242798355e1 * t21213 - 0.11113000182098765433e-1 * t13648 + 0.48897200801234567904e0 * t21216 - 0.13335600218518518519e0 * t21218 - 0.17780800291358024692e0 * t21220;
    (t21999, t22000, t22003, t22007, t22013, t22020, t22059)
}
