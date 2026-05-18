//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 799/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk799<F: Float>(t8013: F, t8026: F, t760: F, t772: F, t4064: F, t7608: F, t2210: F, t2214: F, t2341: F, t3303: F, t3368: F, t3371: F, t3475: F, t3477: F, t3897: F, t4072: F, t7578: F, t7590: F, t773: F, t98: F) -> F {
    let t8027 = t8013 + t8026;
    let t8028 = t760 * t8027;
    let t8029 = t8028 * t772;
    let t8037 = t4064 * t7608;
    let t8045 = -t3303 - F::new(2.9824072957409817) * t773 * t2341 - F::new(2.9824072957409817) * t8029 * t98 - F::new(19.489173774580152) * t3368 - t3371 - F::new(1.6183441301295518) * t3475 - F::new(1.6183441301295518) * t3477 + F::new(2.2140749178833072) * t3897 * t2210 - F::new(2.2140749178833072) * t8037 - F::new(2.2140749178833072) * t4072 * t7590 - F::new(4.4281498357666145) * t4072 * t7578 + F::new(2.2140749178833072) * t3897 * t2214;
    t8045
}
