//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1180/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1180<F: Float>(t12281: F, t21846: F, t4160: F, t1889: F, t5671: F, t5875: F, t16633: F, t2011: F, t833: F, t5440: F, t15865: F, t5426: F, t5661: F, t1363: F, t7028: F, t3738: F, t7037: F) -> (F, F, F, F, F, F, F) {
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
    let t21861 = t4160 * t21860;
    let t21863 = t5426 * t21858;
    let t21864 = t15865 * t21863;
    let t21865 = t5661 * t21864;
    let t21868 = t7028 * t1363;
    let t21871 = t3738 * t7037;
    (t21848, t21852, t21856, t21861, t21865, t21868, t21871)
}
