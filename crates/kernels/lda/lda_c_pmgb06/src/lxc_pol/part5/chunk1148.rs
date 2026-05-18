//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1148/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1148<F: Float>(t132: F, t435: F, t7718: F, t6851: F, t831: F, t161: F, t1639: F, t166: F, t7857: F, t486: F, t7859: F, t13440: F, t20784: F, t20786: F, t20789: F, t20791: F, t20792: F, t20794: F) -> (F, F, F, F, F) {
    let t20796 = t132 * t435 * t7718;
    let t20797 = F::new(2.0) / F::new(15.0) * t20796;
    let t20798 = t831 * t6851;
    let t20799 = F::new(2.0) / F::new(15.0) * t20798;
    let t20803 = t161 * t166 * t1639 * t7857 / F::new(30.0);
    let t20805 = t486 * t7859 / F::new(30.0);
    let t20806 = -t20784 - t20786 - t20789 - t20791 - t20792 - t20794 + t13440 + t20797 + t20799 - t20803 - t20805;
    (t20797, t20799, t20803, t20805, t20806)
}
