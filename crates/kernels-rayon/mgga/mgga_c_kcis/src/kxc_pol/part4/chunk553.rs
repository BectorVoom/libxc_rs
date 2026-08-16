//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 553/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk553(t1022: f64, t2850: f64, t1021: f64, t1020: f64, t359: f64, t982: f64) -> (f64, f64, f64, f64) {
    let t2851 = t1022 * t2850;
    let t2852 = t1021 * t2851;
    let t2853 = t1020 * t2852;
    let t2855 = t982 * t359;
    (t2851, t2852, t2853, t2855)
}
