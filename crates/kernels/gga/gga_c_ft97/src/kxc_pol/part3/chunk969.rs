//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 969/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk969<F: Float>(t10207: F, t10209: F, t10212: F, t13616: F, t1526: F, t15567: F, t18959: F, t18962: F, t18969: F, t18972: F, t18977: F, t18982: F, t2320: F, t342: F, t343: F, t4027: F, t4037: F, t4052: F, t4135: F) -> F {
    let t18986 = t4027 + t4135 + t10207 - t10209 / F::new(36.0) - t10212 / F::new(12.0) - t18959 / F::new(36.0) - t15567 * t18962 / F::new(9.0) - t1526 * t2320 * t4037 / F::new(12.0) + t15567 * t18969 / F::new(6.0) + t1526 * t13616 * t18972 / F::new(6.0) - t18977 / F::new(12.0) - t1526 * t2320 * t4052 / F::new(12.0) - t342 * t343 * t18982 / F::new(4.0);
    t18986
}
