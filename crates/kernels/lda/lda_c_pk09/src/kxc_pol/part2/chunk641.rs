//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 641/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk641<F: Float>(t513: F, t15: F, t902: F, t505: F, t309: F, t6586: F, t4993: F, t514: F, t454: F, t1832: F, t6292: F, t1828: F, t6488: F, t1823: F, t6477: F, t4977: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6932 = t513 * t513;
    let t6933 = 1.0 / t6932;
    let t6938 = t15 * t902;
    let t6944 = t505 * t505;
    let t6945 = 1.0 / t6944;
    let t6950 = t6586 * t309;
    let t6951 = t514 * t4993;
    let t6952 = t454 * t6951;
    let t6954 = 0.08230132705969918 * t6950 * t6952;
    let t6956 = 3.7610742193750633 * t1832 * t6292;
    let t6958 = 1.8805371096875316 * t1828 * t6292;
    let t6962 = 2.507382812916709 * t1832 * t6488;
    let t6964 = t1823 * t6477;
    let t6966 = t514 * t4977;
    (t6933, t6938, t6945, t6954, t6956, t6958, t6962, t6964, t6966)
}
