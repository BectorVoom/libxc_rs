//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 850/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk850<F: Float>(t7498: F, t7526: F, t7544: F, t7573: F, t7629: F, t7731: F, t7814: F, t7848: F, t41: F, t7337: F, t163: F, t164: F, t169: F, t171: F, t4260: F, t4265: F, t4275: F, t4279: F, t5442: F, t5444: F, t5449: F, t5468: F, t5472: F, t5479: F) -> (F, F, F) {
    let t7851 = t7498 + t7526 + t7544 + t7573 + t7629 + t7731 + t7814 + t7848;
    let t7856 = t41 * t7337;
    let t7862 = -t4260 - t4265 + t4275 - t4279 - F::new(0.09451622166942335) * t5444 + F::new(0.1890324433388467) * t5449 - F::new(0.1890324433388467) * t5442 - F::new(0.005388405304614574) * t169 * t171 * t7851 * t163 - F::new(0.031505407223141116) * t7856 * t164 - F::new(0.0014862827083471494) * t5468 + F::new(0.01975389032890948) * t5472 - F::new(0.01185233419734569) * t5479;
    (t7851, t7856, t7862)
}
