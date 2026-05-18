//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1081/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1081<F: Float>(t2173: F, t2175: F, t26728: F, t26732: F, t26736: F, t26739: F, t26742: F, t26745: F, t26748: F, t26751: F, t26755: F, t26758: F, t26764: F, t26767: F, t26774: F, t7687: F, t7690: F, t7693: F, t7703: F, t7706: F) -> F {
    let t26776 = F::new(0.13901041666666666667e-2) * t7687 * t7693 + F::new(0.18550940104166666667e-3) * t26728 * t7693 + F::new(0.92754700520833333333e-4) * t7690 * t26732 + F::new(0.69505208333333333333e-3) * t2173 * t26736 - F::new(0.4946917361111111111e-3) * t26739 * t7693 - F::new(0.67960648148148148147e-2) * t26742 * t2175 + F::new(0.12356481481481481482e-2) * t26745 - F::new(0.46336805555555555556e-3) * t26748 * t7706 + F::new(0.22109259259259259258e-2) * t26751 + F::new(0.33163888888888888888e-2) * t26755 - F::new(0.15445601851851851852e-3) * t26758 - F::new(0.33163888888888888888e-2) * t26764 - F::new(0.23168402777777777778e-3) * t7703 * t26767 + F::new(0.69505208333333333333e-3) * t2173 * t26732 + F::new(0.49745833333333333332e-2) * t26774;
    t26776
}
