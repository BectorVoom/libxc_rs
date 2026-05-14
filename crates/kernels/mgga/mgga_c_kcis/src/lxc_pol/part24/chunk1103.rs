//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1103/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1103<F: Float>(t1851: F, t26996: F, t5329: F, t5341: F, t1267: F, t6737: F, t92735: F, t100130: F, t100133: F, t100136: F, t100139: F, t100142: F, t100145: F, t100148: F, t100152: F, t7788: F, t96787: F) -> (F, F, F) {
    let t100157 = t5329 * t26996 * t1851 * t5341;
    let t100162 = t5329 * t92735 * t6737 * t1267;
    let t100165 = -t96787 - 0.23168402777777777778e-3 * t100130 + 0.61905925925925925925e-2 * t100133 + 0.19345601851851851852e-2 * t100136 + 0.7722800925925925926e-4 * t100139 - 0.19345601851851851852e-2 * t100142 + 0.12897067901234567901e-2 * t100145 - 0.11607361111111111111e-1 * t100148 + 0.69505208333333333334e-3 * t7788 * t100152 - 0.69505208333333333334e-3 * t7788 * t100157 + 0.208515625e-2 * t7788 * t100162;
    (t100157, t100162, t100165)
}
