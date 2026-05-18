//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 1006/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk1006<F: Float>(t299: F, t14568: F, t15554: F, t10947: F, t10948: F, t10949: F, t10950: F, t12091: F, t13: F, t13291: F, t14389: F, t1939: F, t2316: F, t2635: F, t2973: F) -> F {
    let t300 = F::new(10000000.0) <= t299;
    let t15556 = piecewise3::<f64>(t300, F::new(0.0), t14568 + t15554);
    let tv3rho31 = t1939 + t2316 + t2635 + t2973 + t10947 + t10948 + t10949 + t10950 + t13 * (t12091 + t13291 + t14389 + t15556);
    tv3rho31
}
