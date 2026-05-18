//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 534/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk534<F: Float>(t1158: F, t1165: F, t1169: F, t1181: F, t1189: F, t1195: F, t1740: F, t1885: F, t1898: F, t2311: F, t2313: F, t2680: F) -> F {
    let t2685 = -F::new(0.0005811348303577384) * t1898 - F::new(0.02394846802050922) * t2311 + F::new(0.039914113367515366) * t2313 - F::new(0.10809180959278285) * t1885 + t1158 - t1165 + t1169 - t1181 - t1189 + t1195 - t1740;
    let t2686 = t2680 + t2685;
    t2686
}
