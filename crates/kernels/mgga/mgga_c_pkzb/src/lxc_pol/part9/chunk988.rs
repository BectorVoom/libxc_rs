//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 988/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk988<F: Float>(t7884: F, t790: F, t1134: F, t1144: F, t2112: F, t2120: F, t2146: F, t2957: F, t2965: F, t2990: F, t307: F, t311: F, t7805: F, t7821: F, t7825: F, t7828: F, t786: F, t800: F) -> (F, F) {
    let t7885 = t790 * t7884;
    let t7888 = F::cast_from(0.65854491829355115987e0_f64) * t7805 * t311 - F::cast_from(0.13170898365871023197e1_f64) * t2957 * t800 + F::cast_from(0.13170898365871023197e1_f64) * t1134 * t2120 - F::cast_from(0.65854491829355115987e0_f64) * t1134 * t2146 - F::cast_from(0.65854491829355115987e0_f64) * t2112 * t1144 + F::cast_from(0.26341796731742046394e1_f64) * t786 * t2965 - F::cast_from(0.13170898365871023197e1_f64) * t786 * t2990 - F::cast_from(0.39512695097613069591e1_f64) * t307 * t7821 + F::cast_from(0.26341796731742046394e1_f64) * t307 * t7825 + F::cast_from(0.13170898365871023197e1_f64) * t307 * t7828 - F::cast_from(0.65854491829355115987e0_f64) * t307 * t7885;
    (t7885, t7888)
}
