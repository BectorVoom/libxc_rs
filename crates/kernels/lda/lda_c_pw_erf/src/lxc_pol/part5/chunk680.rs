//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 680/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk680<F: Float>(t1: F, t2343: F, t397: F, t2951: F, t2990: F, t5695: F, t2748: F, t2752: F, t2755: F, t2759: F, t2761: F, t2944: F, t2950: F, t2989: F, t5687: F, t5689: F, t5979: F, t6013: F) -> (F, F, F) {
    let t6055 = t2343 * t1;
    let t6056 = t6055 * t397;
    let t6057 = F::cast_from(0.0001831155503675316_f64) * t6056;
    let t6058 = F::cast_from(1.169644679491041_f64) * t2951;
    let t6059 = F::cast_from(17.315755899375862_f64) * t2990;
    let t6060 = F::cast_from(16.0_f64) * t5695;
    let t6061 = t5979 + t6013 - t2748 + t2752 - t2755 + t2759 - t2761 - t6057 - t2944 + t2950 + t6058 + t5687 - t5689 - t2989 - t6059 - t6060;
    (t6055, t6056, t6061)
}
