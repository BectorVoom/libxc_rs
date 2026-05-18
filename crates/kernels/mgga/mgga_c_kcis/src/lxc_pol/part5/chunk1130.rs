//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1130/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1130<F: Float>(t19040: F, t261: F, t3005: F, t6423: F, t1226: F, t15351: F, t4763: F, t18645: F, t18661: F, t18669: F, t18674: F, t18679: F, t18683: F, t18828: F, t18830: F, t18833: F, t18835: F, t18904: F) -> (F, F, F, F) {
    let t19042 = F::new(0.62182e-1) * t19040 * t261;
    let t19043 = t3005 * t6423;
    let t19044 = t19043 * t1226;
    let t19047 = t4763 * t15351;
    let t19071 = F::new(0.258925e1) * t18835 + F::new(0.19419375e1) * t18828 - F::new(0.258925e1) * t18830 - F::new(0.1294625e1) * t18833 - F::new(0.20128333333333333333e0) * t18674 + F::new(0.60385e0) * t18679 + F::new(0.67094444444444444443e-1) * t18645 - F::new(0.20128333333333333333e0) * t18661 + F::new(0.10064166666666666667e0) * t18669 - F::new(0.301925e0) * t18683 + F::new(0.16557e0) * t18904;
    (t19042, t19044, t19047, t19071)
}
