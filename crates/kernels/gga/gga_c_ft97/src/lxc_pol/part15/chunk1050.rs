//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1050/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1050<F: Float>(t15128: F, t22405: F, t10683: F, t10703: F, t1212: F, t1248: F, t15402: F, t1901: F, t21978: F, t22346: F, t296: F, t319: F, t4139: F, t446: F, t4969: F, t5225: F, t5299: F, t5330: F, t72523: F, t835: F, t840: F, t84734: F, t84740: F, t84767: F, t871: F, t88105: F, t88756: F) -> (F, F) {
    let t91005 = t15128 * t22405;
    let t91015 = -8.0 / 27.0 * t84734 + 4.0 / 9.0 * t84740 + 2.0 / 3.0 * t446 * t835 * t319 * t88756 + 8.0 * t446 * t10683 * t871 * t21978 * t1248 - 12.0 * t446 * t10683 * t319 * t5225 * t5299 + 8.0 / 3.0 * t1901 * t10703 * t4969 * t5330 + 4.0 / 3.0 * t446 * t840 * t871 * t22346 * t1212 + 8.0 * t446 * t296 * t91005 - 8.0 / 9.0 * t84767 + 16.0 / 27.0 * t72523 + 8.0 / 3.0 * t1901 * t4139 * t15402 * t88105;
    (t91005, t91015)
}
