//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1159/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1159<F: Float>(t10403: F, t10416: F, t1447: F, t5451: F, t5454: F, t5458: F, t5499: F, t1920: F, t3226: F, t5464: F, t5467: F, t5471: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13834 = F::new(4.0) / F::new(45.0) * t10403;
    let t13835 = F::new(4.0) / F::new(45.0) * t10416;
    let t13836 = t1447 * t5451;
    let t13837 = F::new(4.0) / F::new(45.0) * t13836;
    let t13838 = t1447 * t5454;
    let t13839 = F::new(4.0) / F::new(9.0) * t13838;
    let t13840 = t5499 * t5458;
    let t13841 = F::new(4.0) / F::new(9.0) * t13840;
    let t13842 = t3226 * t1920;
    let t13843 = F::new(4.0) / F::new(27.0) * t13842;
    let t13844 = t1447 * t5464;
    let t13845 = F::new(4.0) / F::new(27.0) * t13844;
    let t13846 = t1447 * t5467;
    let t13847 = F::new(2.0) / F::new(27.0) * t13846;
    let t13848 = t1447 * t5471;
    (t13834, t13835, t13837, t13839, t13841, t13843, t13845, t13847, t13848)
}
