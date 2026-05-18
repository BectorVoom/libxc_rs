//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 966/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk966<F: Float>(t297: F, t9973: F, t1633: F, t10240: F, t10244: F, t10249: F, t1451: F, t1562: F, t1597: F, t1609: F, t2587: F, t328: F, t5865: F, t5868: F, t5871: F, t5880: F, t5884: F) -> F {
    let t10251 = t9973 * t297;
    let t10252 = t10251 * t1633;
    let t10255 = t5865 / F::new(6.0) + t5868 / F::new(6.0) + t5871 - t5880 - t5884 - t1597 * t2587 / F::new(6.0) - t328 * t10240 / F::new(6.0) - t1609 * t10244 / F::new(12.0) + t1562 * t2587 / F::new(6.0) - F::new(0.037002892246025966) * t10249 - t10252 * t1451 / F::new(6.0);
    t10255
}
