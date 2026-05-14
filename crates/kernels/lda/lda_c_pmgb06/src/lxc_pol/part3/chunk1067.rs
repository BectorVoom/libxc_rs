//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1067/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1067<F: Float>(t117: F, t123: F, t2360: F, t740: F, t10795: F, t10799: F, t10802: F, t10806: F, t10808: F, t10811: F, t10813: F, t10817: F, t10820: F, t10823: F, t10825: F, t10828: F, t10831: F, t10834: F, t10838: F) -> (F,) {
    let t14500 = t123 * t740 * t2360 * t117;
    let t14501 = 0.07184540406152766 * t14500;
    let t14511 = -t14501 + 0.010403978958430045 * t10795 - 0.0014862827083471494 * t10799 - 0.004458848125041448 * t10802 - t10806 - t10808 - t10811 - 0.01777850129601853 * t10813 + t10817 - 0.001975389032890948 * t10820 - 0.01185233419734569 * t10823 - 0.07769863529371063 * t10825 - t10828 + 0.01975389032890948 * t10831 + 0.059261670986728444 * t10834 + t10838;
    (t14511,)
}
