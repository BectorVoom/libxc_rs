//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 859/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk859<F: Float>(t849: F, t8881: F, t164: F, t62: F, t7766: F, t896: F, t890: F, t192: F, t200: F, t4684: F, t8049: F, t8849: F, t8851: F, t8859: F, t8863: F, t8867: F, t8871: F, t8875: F, t8879: F) -> (F, F, F) {
    let t8882 = t849 * t8881;
    let t8883 = t164 * t8882;
    let t8885 = t62 * t7766;
    let t8886 = t896 * t8885;
    let t8887 = t890 * t8886;
    let t8890 = -F::new(0.027433775686566395) * t8849 - F::new(0.027433775686566395) * t8851 - F::new(2.427516195194328) * t200 * t8049 - F::new(2.2140749178833072) * t192 * t8049 + F::new(1.800081713982063) * t8859 - F::new(1.800081713982063) * t8863 - F::new(1.800081713982063) * t8867 + F::new(22.07984838129906) * t8871 + F::new(22.07984838129906) * t8875 + F::new(22.07984838129906) * t8879 + F::new(22.07984838129906) * t8883 + F::new(1.800081713982063) * t8887 - F::new(1.2536914064583544) * t4684;
    (t8883, t8887, t8890)
}
