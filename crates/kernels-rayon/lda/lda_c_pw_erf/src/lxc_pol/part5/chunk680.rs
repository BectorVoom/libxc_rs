//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 680/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk680(t1: f64, t2343: f64, t397: f64, t2951: f64, t2990: f64, t5695: f64, t2748: f64, t2752: f64, t2755: f64, t2759: f64, t2761: f64, t2944: f64, t2950: f64, t2989: f64, t5687: f64, t5689: f64, t5979: f64, t6013: f64) -> (f64, f64, f64) {
    let t6055 = t2343 * t1;
    let t6056 = t6055 * t397;
    let t6057 = 0.0001831155503675316_f64 * t6056;
    let t6058 = 1.169644679491041_f64 * t2951;
    let t6059 = 17.315755899375862_f64 * t2990;
    let t6060 = 16.0_f64 * t5695;
    let t6061 = t5979 + t6013 - t2748 + t2752 - t2755 + t2759 - t2761 - t6057 - t2944 + t2950 + t6058 + t5687 - t5689 - t2989 - t6059 - t6060;
    (t6055, t6056, t6061)
}
