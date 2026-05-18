//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 967/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk967<F: Float>(t13714: F, t13717: F, t13912: F, t15397: F, t15411: F, t18650: F, t18655: F, t18659: F, t18664: F, t18667: F, t18877: F, t18880: F, t18885: F, t18887: F, t18890: F, t18909: F, t18912: F, t18920: F, t20431: F, t20452: F, t9691: F, t9708: F) -> F {
    let t20454 = -F::new(0.22954444444444444444e0) * t9691 - F::new(0.11577222222222222222e0) * t9708 + F::new(0.23154444444444444445e-1) * t18877 - F::new(0.104195e0) * t18880 - t15397 + F::new(0.4630888888888888889e-1) * t13912 + F::new(0.68863333333333333332e0) * t13717 - F::new(0.157790625e0) * t18885 + F::new(0.6311625e0) * t18887 + F::new(0.31558125e0) * t18890 + t20431 - F::new(0.46308888888888888889e-1) * t18909 - F::new(0.13892666666666666667e0) * t18912 + t15411 - F::new(0.68863333333333333332e0) * t13714 - F::new(0.57386111111111111112e0) * t18650 + F::new(0.20659e1) * t18655 - F::new(0.13772666666666666667e1) * t18659 - F::new(0.309885e1) * t18664 + F::new(0.41318e1) * t18667 + F::new(0.6311625e0) * t18920 + t20452;
    t20454
}
