//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 979/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk979<F: Float>(t77716: F, t71340: F, t8571: F, t3219: F, t9090: F, t75662: F, t75664: F, t2144: F, t3351: F, t498: F, t7231: F, t9540: F) -> (F, F, F, F, F, F) {
    let t77717 = F::cast_from(0.42564599893297839398e-5_f64) * t77716;
    let t77718 = t8571 * t71340;
    let t77719 = F::cast_from(0.12769379967989351819e-4_f64) * t77718;
    let t77723 = t9090 * t3219;
    let t77724 = F::cast_from(0.99317399751028291929e-5_f64) * t77723;
    let t77725 = F::cast_from(0.3830813990396805546e-4_f64) * t75662;
    let t77726 = F::cast_from(0.1276937996798935182e-4_f64) * t75664;
    let t77732 = t3351 * t7231 * t2144 * t9540 * t498;
    (t77717, t77719, t77724, t77725, t77726, t77732)
}
