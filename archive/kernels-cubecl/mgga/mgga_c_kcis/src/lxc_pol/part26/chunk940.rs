//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 940/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk940<F: Float>(t1307: F, t6917: F, t12281: F, t4160: F, t1889: F, t5671: F, t5875: F, t16633: F, t2011: F, t833: F, t5440: F, t15865: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21846 = t6917 * t1307;
    let t21847 = t12281 * t21846;
    let t21848 = t4160 * t21847;
    let t21850 = t1889 * t5671;
    let t21851 = t12281 * t21850;
    let t21852 = t4160 * t21851;
    let t21854 = t1889 * t5875;
    let t21855 = t16633 * t21854;
    let t21856 = t4160 * t21855;
    let t21858 = t2011 * t833;
    let t21859 = t5440 * t21858;
    let t21860 = t15865 * t21859;
    (t21846, t21848, t21850, t21852, t21854, t21856, t21858, t21859, t21860)
}
