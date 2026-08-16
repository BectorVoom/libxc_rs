//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1320/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1320(t1307: f64, t6917: f64, t12281: f64, t4160: f64, t1889: f64, t5671: f64, t5875: f64, t16633: f64, t2011: f64, t833: f64, t5440: f64, t15865: f64) -> (f64, f64, f64, f64, f64) {
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
    (t21848, t21852, t21856, t21858, t21860)
}
