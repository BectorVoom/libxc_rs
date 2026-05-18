//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1396/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1396<F: Float>(t18137: F, t737: F, t110: F, t2105: F, t1599: F, t18093: F, t18096: F, t18100: F, t18105: F, t18110: F, t18116: F, t18121: F, t18125: F, t18130: F, t18133: F, t4439: F) -> F {
    let t18138 = t737 * t18137;
    let t18141 = t110 * t2105;
    let t18142 = t1599 * t18141;
    let t18144 = -t18093 - t4439 * t18096 / F::new(288.0) - t4439 * t18100 / F::new(576.0) + t4439 * t18105 / F::new(288.0) - t4439 * t18110 / F::new(432.0) - t1599 * t18116 / F::new(192.0) + t4439 * t18121 / F::new(144.0) - t4439 * t18125 / F::new(576.0) + t4439 * t18130 / F::new(144.0) - t4439 * t18133 / F::new(288.0) - t1599 * t18138 / F::new(288.0) + t18142 / F::new(864.0);
    t18144
}
